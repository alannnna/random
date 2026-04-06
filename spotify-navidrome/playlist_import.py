#!/usr/bin/env python3
"""
Spotify → Navidrome playlist importer.

Commands:
  config               Set up Navidrome connection (saved to config.json)
  import [--file F]    Import a playlist (paste/pipe JSON, or pass a file)
  status <file>        Show import progress for a previously started import
  retry  <file>        Retry failed tracks (--song N to retry one)
"""

import argparse
import getpass
import hashlib
import json
import secrets
import sys
from datetime import datetime
from pathlib import Path

import requests

CONFIG_FILE = Path(__file__).parent / "config.json"


# ── Config ────────────────────────────────────────────────────────────────────

def load_config():
    if not CONFIG_FILE.exists():
        sys.exit("No config found. Run:  python playlist_import.py config")
    return json.loads(CONFIG_FILE.read_text())


def cmd_config(_args):
    existing = json.loads(CONFIG_FILE.read_text()) if CONFIG_FILE.exists() else {}
    print("Navidrome configuration (press Enter to keep existing value)\n")

    def prompt(label, key, secret=False):
        cur = existing.get(key, "")
        hint = f"[{'*' * len(cur)}]" if (secret and cur) else (f"[{cur}]" if cur else "")
        fn = getpass.getpass if secret else input
        val = fn(f"  {label} {hint}: ").strip()
        return val or cur

    cfg = {
        "url":      prompt("Navidrome URL", "url"),
        "username": prompt("Username",      "username"),
        "password": prompt("Password",      "password", secret=True),
    }
    CONFIG_FILE.write_text(json.dumps(cfg, indent=2))
    print(f"\nConfig saved to {CONFIG_FILE}")


# ── Subsonic API ──────────────────────────────────────────────────────────────

def _auth_params(cfg):
    salt  = secrets.token_hex(8)
    token = hashlib.md5((cfg["password"] + salt).encode()).hexdigest()
    return [
        ("u", cfg["username"]),
        ("t", token),
        ("s", salt),
        ("v", "1.16.1"),
        ("c", "spotify-navidrome"),
        ("f", "json"),
    ]


def _check(body):
    if body.get("status") != "ok":
        err = body.get("error", {})
        raise RuntimeError(f"Subsonic error {err.get('code')}: {err.get('message')}")
    return body


def _api(cfg, endpoint, extra=None):
    params = dict(_auth_params(cfg))
    if extra:
        params.update(extra)
    url = cfg["url"].rstrip("/") + f"/rest/{endpoint}"
    r = requests.get(url, params=params, timeout=15)
    r.raise_for_status()
    return _check(r.json().get("subsonic-response", {}))


def _api_multi(cfg, endpoint, param_list):
    """For endpoints that need repeated query params (e.g. multiple songId)."""
    all_params = _auth_params(cfg) + param_list
    url = cfg["url"].rstrip("/") + f"/rest/{endpoint}"
    r = requests.get(url, params=all_params, timeout=15)
    r.raise_for_status()
    return _check(r.json().get("subsonic-response", {}))


def search_songs(cfg, title, artist, album):
    """
    Search Navidrome for a song. Returns (list_of_matches, error_string_or_None).
    Results are ranked: exact title+artist first, then fuzzy, then album bonus.
    """
    query = " ".join(filter(None, [title, artist]))
    try:
        resp = _api(cfg, "search3", {
            "query": query,
            "songCount": 20,
            "albumCount": 0,
            "artistCount": 0,
        })
        songs = resp.get("searchResult3", {}).get("song", [])
    except Exception as e:
        return [], str(e)

    if not songs:
        return [], None

    def score(s):
        t = s.get("title", "").lower()
        a = s.get("artist", "").lower()
        al = s.get("album", "").lower()
        n = 0
        if title.lower() == t:            n += 5
        elif title.lower() in t or t in title.lower():
            n += 2
        if artist.lower() == a:          n += 4
        elif artist.lower() in a or a in artist.lower():
            n += 2
        if album and (album.lower() in al or al in album.lower()):
            n += 1
        return n

    ranked = sorted(songs, key=score, reverse=True)
    # Keep only results with at least a partial title+artist match
    good = [s for s in ranked if score(s) >= 3]
    return good, None


def get_playlists(cfg):
    resp = _api(cfg, "getPlaylists")
    return resp.get("playlists", {}).get("playlist", []) or []


def create_playlist(cfg, name):
    resp = _api(cfg, "createPlaylist", {"name": name})
    return resp["playlist"]["id"]


def rebuild_playlist(cfg, playlist_id, playlist_name, ordered_song_ids):
    """
    Replace the entire playlist content in one shot using createPlaylist
    with playlistId, which is the idiomatic Subsonic way to update a playlist.
    """
    params = [("playlistId", playlist_id), ("name", playlist_name)]
    params += [("songId", sid) for sid in ordered_song_ids]
    _api_multi(cfg, "createPlaylist", params)


# ── Spotify JSON parsing ──────────────────────────────────────────────────────

def parse_spotify_json(data):
    """
    Accepts several common shapes:

    1. Official Spotify Web API playlist object:
       {"name": "...", "tracks": {"items": [{"track": {"name", "artists", "album"}}]}}

    2. Simple list of track objects:
       [{"name": "...", "artists": [{"name": "..."}], "album": {"name": "..."}}]

    3. Exportify-style JSON (converted from their CSV export):
       [{"Track Name": "...", "Artist Name(s)": "...", "Album Name": "..."}]

    Returns (playlist_name, [{"title", "artist", "album"}])
    """
    if isinstance(data, dict):
        name = data.get("name", "Imported Playlist")
        tracks_obj = data.get("tracks", {})
        items = tracks_obj.get("items", []) if isinstance(tracks_obj, dict) else (tracks_obj or [])
        tracks = []
        for item in items:
            t = item.get("track") if isinstance(item, dict) and "track" in item else item
            if not t:
                continue
            artists = t.get("artists", [])
            artist = artists[0].get("name", "") if artists else t.get("artist", "")
            album_field = t.get("album", {})
            album = album_field.get("name", "") if isinstance(album_field, dict) else str(album_field)
            tracks.append({
                "title":  t.get("name", ""),
                "artist": artist,
                "album":  album,
            })
        return name, tracks

    if isinstance(data, list):
        tracks = []
        for item in data:
            # Exportify column names or simple keys
            title  = (item.get("Track Name") or item.get("name")   or item.get("title",  "")).strip()
            artist = (item.get("Artist Name(s)") or item.get("Artists") or
                      item.get("artist") or "").strip()
            album  = (item.get("Album Name") or item.get("album")  or "").strip()
            tracks.append({"title": title, "artist": artist, "album": album})
        return "Imported Playlist", tracks

    raise ValueError("Unrecognized Spotify JSON shape. Expected a dict or list.")


# ── State file ────────────────────────────────────────────────────────────────

def state_path(source_file):
    p = Path(source_file)
    return p.parent / (p.stem + ".import.json")


def load_state(source_file):
    sp = state_path(source_file)
    if sp.exists():
        return json.loads(sp.read_text()), sp
    return None, sp


def save_state(sp, state):
    Path(sp).write_text(json.dumps(state, indent=2))


def init_state(sp, playlist_name, tracks, navidrome_playlist_id):
    state = {
        "playlist_name":         playlist_name,
        "navidrome_playlist_id": navidrome_playlist_id,
        "tracks": [
            {
                "index":        i,
                "title":        t["title"],
                "artist":       t["artist"],
                "album":        t["album"],
                "status":       "pending",  # pending | ok | failed | skipped
                "navidrome_id": None,
                "error":        None,
            }
            for i, t in enumerate(tracks)
        ],
    }
    save_state(sp, state)
    return state


# ── Interactive matching ──────────────────────────────────────────────────────

def pick_song(cfg, track):
    """
    Search for a track and return (navidrome_id, error).
    Prompts the user when there are ambiguous results.
    """
    results, err = search_songs(cfg, track["title"], track["artist"], track["album"])

    if err:
        return None, f"search error: {err}"
    if not results:
        return None, "not found in library"
    if len(results) == 1:
        return results[0]["id"], None

    # Check if the best result is an unambiguous exact match
    top = results[0]
    title_exact  = top.get("title", "").lower()  == track["title"].lower()
    artist_match = track["artist"].lower() in top.get("artist", "").lower()
    if title_exact and artist_match and len(results) >= 2:
        # Is the second result also an exact match?
        r2 = results[1]
        r2_title_exact  = r2.get("title", "").lower()  == track["title"].lower()
        r2_artist_match = track["artist"].lower() in r2.get("artist", "").lower()
        if not (r2_title_exact and r2_artist_match):
            # Top result is uniquely best — take it silently
            return top["id"], None

    # Ambiguous — ask the user
    print()
    print(f"    Ambiguous: {track['title']} — {track['artist']}")
    candidates = results[:6]
    for i, s in enumerate(candidates, 1):
        year  = s.get("year", "?")
        print(f"      [{i}] {s.get('title')} — {s.get('artist')}  ({s.get('album', '?')}, {year})")
    print(f"      [0] Skip this track")

    while True:
        try:
            raw = input("    Choose [1]: ").strip()
        except (EOFError, KeyboardInterrupt):
            return None, "skipped (interrupted)"
        choice = int(raw) if raw.isdigit() else 1
        if choice == 0:
            return None, "skipped by user"
        if 1 <= choice <= len(candidates):
            return candidates[choice - 1]["id"], None


# ── Core import loop ──────────────────────────────────────────────────────────

def do_import(cfg, state, sp):
    """Process all pending tracks, save state after each, then sync the playlist."""
    pending = [t for t in state["tracks"] if t["status"] == "pending"]
    total   = len(state["tracks"])

    if not pending:
        print("  Nothing pending.")
        return

    for track in pending:
        idx   = track["index"]
        label = f"{track['title']} — {track['artist']}"
        print(f"  [{idx+1:3}/{total}]  {label[:65]:<65}", end="  ", flush=True)

        nid, err = pick_song(cfg, track)

        if nid:
            track["status"]       = "ok"
            track["navidrome_id"] = nid
            track["error"]        = None
            print("✓")
        else:
            track["status"] = "failed"
            track["error"]  = err
            print(f"✗  ({err})")

        save_state(sp, state)

    _sync_playlist(cfg, state, sp)


def _sync_playlist(cfg, state, sp):
    """Push all currently matched songs (in original order) to Navidrome."""
    ordered_ids = [
        t["navidrome_id"]
        for t in sorted(state["tracks"], key=lambda x: x["index"])
        if t["status"] == "ok" and t["navidrome_id"]
    ]
    if not ordered_ids:
        print("\n  No matched tracks to sync.")
        return
    try:
        rebuild_playlist(cfg, state["navidrome_playlist_id"],
                         state["playlist_name"], ordered_ids)
        ok     = sum(1 for t in state["tracks"] if t["status"] == "ok")
        failed = sum(1 for t in state["tracks"] if t["status"] == "failed")
        print(f"\n  Playlist synced: {ok} tracks in Navidrome. {failed} failed.")
    except Exception as e:
        print(f"\n  Warning: failed to sync playlist to Navidrome: {e}")


# ── Commands ──────────────────────────────────────────────────────────────────

def cmd_import(args):
    cfg = load_config()

    # ── Read JSON ──
    if args.file:
        source_file = Path(args.file)
        if not source_file.exists():
            sys.exit(f"File not found: {args.file}")
        raw = source_file.read_text()
    else:
        if sys.stdin.isatty():
            print("Paste your Spotify JSON below, then press Ctrl+D on a new line:")
            print()
        raw = sys.stdin.read()

    try:
        data = json.loads(raw)
    except json.JSONDecodeError as e:
        sys.exit(f"Invalid JSON: {e}")

    playlist_name, tracks = parse_spotify_json(data)

    if not tracks:
        sys.exit("No tracks found in the JSON.")

    # ── Save JSON to file if it came from stdin ──
    if args.file:
        source_file = Path(args.file)
    else:
        safe = "".join(c if c.isalnum() or c in " -_" else "_" for c in playlist_name).strip()
        ts   = datetime.now().strftime("%Y%m%d_%H%M%S")
        source_file = Path(f"{safe}_{ts}.json")
        source_file.write_text(raw)
        print(f"Playlist JSON saved to: {source_file}")

    sp = state_path(source_file)
    print(f"State file:              {sp}\n")

    # ── Playlist name ──
    if args.name:
        playlist_name = args.name
    else:
        entered = input(f"Playlist name [{playlist_name}]: ").strip()
        if entered:
            playlist_name = entered

    # ── Check for existing state / existing playlist ──
    state, _ = load_state(source_file)

    if state:
        pending = sum(1 for t in state["tracks"] if t["status"] == "pending")
        failed  = sum(1 for t in state["tracks"] if t["status"] == "failed")
        print(f'Resuming import for "{state["playlist_name"]}" '
              f'({pending} pending, {failed} failed)\n')
    else:
        print(f'Checking Navidrome for existing playlist "{playlist_name}"... ', end="", flush=True)
        existing = get_playlists(cfg)
        match = next((p for p in existing if p.get("name") == playlist_name), None)
        if match:
            print(f"found (id: {match['id']}), will update.")
            nav_id = match["id"]
        else:
            print("not found, creating.")
            nav_id = create_playlist(cfg, playlist_name)

        state = init_state(sp, playlist_name, tracks, nav_id)

    print(f"Importing {sum(1 for t in state['tracks'] if t['status'] == 'pending')} "
          f"of {len(state['tracks'])} tracks...\n")

    do_import(cfg, state, sp)

    failed = [t for t in state["tracks"] if t["status"] == "failed"]
    if failed:
        print(f"\n  {len(failed)} track(s) failed. Review with:")
        print(f"    python playlist_import.py status {sp}")
        print(f"  Retry all with:")
        print(f"    python playlist_import.py retry {sp}")


def cmd_status(args):
    source, sp = _resolve(args.file)

    if not sp.exists():
        sys.exit(f"No state file found. Expected: {sp}")

    state = json.loads(sp.read_text())

    ok      = [t for t in state["tracks"] if t["status"] == "ok"]
    failed  = [t for t in state["tracks"] if t["status"] == "failed"]
    pending = [t for t in state["tracks"] if t["status"] == "pending"]

    print(f'\nPlaylist : "{state["playlist_name"]}"')
    print(f'Navidrome: id {state["navidrome_playlist_id"]}')
    print(f'Progress : {len(ok)}/{len(state["tracks"])} matched'
          f' — {len(failed)} failed, {len(pending)} pending\n')

    if failed:
        print("Failed tracks:")
        for t in failed:
            print(f"  [{t['index']:3}]  {t['title'][:40]:<40}  {t['artist']:<25}  → {t['error']}")

    if pending:
        print(f"\n{len(pending)} track(s) still pending (not yet attempted).")

    if not failed and not pending:
        print("All tracks matched successfully.")


def cmd_retry(args):
    cfg = load_config()
    source, sp = _resolve(args.file)

    if not sp.exists():
        sys.exit(f"No state file found. Expected: {sp}")

    state = json.loads(sp.read_text())

    if args.song is not None:
        # Retry a single track by index
        matches = [t for t in state["tracks"] if t["index"] == args.song]
        if not matches:
            sys.exit(f"No track with index {args.song}. Check `status` for valid indices.")
        track = matches[0]
        if track["status"] == "ok":
            print(f"Track {args.song} already succeeded: {track['title']} — {track['artist']}")
            return
        track["status"] = "pending"
        track["error"]  = None
        save_state(sp, state)
        print(f"Retrying track {args.song}: {track['title']} — {track['artist']}\n")
    else:
        # Retry all failed
        to_retry = [t for t in state["tracks"] if t["status"] == "failed"]
        if not to_retry:
            print("No failed tracks to retry.")
            pending = sum(1 for t in state["tracks"] if t["status"] == "pending")
            if pending:
                print(f"There are {pending} pending tracks — run import to process them.")
            return
        print(f"Retrying {len(to_retry)} failed track(s)...\n")
        for t in to_retry:
            t["status"] = "pending"
            t["error"]  = None
        save_state(sp, state)

    do_import(cfg, state, sp)


# ── Helpers ───────────────────────────────────────────────────────────────────

def _resolve(path_str):
    """
    Given either a source .json file or a .import.json state file,
    return (source_path, state_path).
    """
    p = Path(path_str)
    if path_str.endswith(".import.json"):
        # Strip the ".import" part to recover the source stem
        stem   = p.name[: -len(".import.json")]
        source = p.parent / (stem + ".json")
        sp     = p
    else:
        source = p
        sp     = state_path(p)
    return source, sp


# ── Entry point ───────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="Import Spotify playlists into Navidrome via the Subsonic API.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser("config", help="Configure Navidrome connection")

    p_import = sub.add_parser("import", help="Import a Spotify playlist JSON")
    p_import.add_argument(
        "--file", "-f",
        help="Path to a saved Spotify JSON file (omit to paste/pipe JSON directly)",
    )
    p_import.add_argument(
        "--name", "-n",
        help="Override the playlist name",
    )

    p_status = sub.add_parser("status", help="Show import progress")
    p_status.add_argument("file", help="Source .json or .import.json file")

    p_retry = sub.add_parser("retry", help="Retry failed tracks")
    p_retry.add_argument("file", help="Source .json or .import.json file")
    p_retry.add_argument(
        "--song", "-s", type=int, metavar="INDEX",
        help="Retry a single track by its index (shown in `status`)",
    )

    args = parser.parse_args()

    dispatch = {
        "config": cmd_config,
        "import": cmd_import,
        "status": cmd_status,
        "retry":  cmd_retry,
    }
    dispatch[args.command](args)


if __name__ == "__main__":
    main()
