#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BOLD='\033[1m'
RESET='\033[0m'

# Module definitions: "dir:goal:ex1,ex2,..."
declare -a MODULES=(
    "module_01_data_structures:I can implement fundamental data structures:ex01_linked_list,ex02_stack,ex03_ring_buffer,ex04_hash_map,ex05_lru_cache"
    "module_02_threads:I can safely share data between threads:ex01_data_race,ex02_deadlock,ex03_condition_var,ex04_atomics,ex05_rwlock"
    "module_03_concurrency_patterns:I can build concurrent abstractions like thread pools and barriers:ex01_bounded_queue,ex02_semaphore,ex03_barrier,ex04_thread_pool,ex05_pipeline"
    "module_04_io_caching:I can design efficient I/O with buffering and caching:ex01_buffered_writer,ex02_line_reader,ex03_write_through_cache,ex04_lru_cache_threadsafe,ex05_io_system"
)

# Configure + build everything once, silently
echo ""
echo -e "${BOLD}Building...${RESET}"
cmake -S "$SCRIPT_DIR" -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE=Debug -Wno-dev \
    > "$BUILD_DIR/cmake_configure.log" 2>&1 || true
cmake --build "$BUILD_DIR" --parallel \
    > "$BUILD_DIR/cmake_build.log" 2>&1 || true
echo ""

check_exercise() {
    local dir="$1"
    local ex="$2"
    local bin="$BUILD_DIR/$dir/$ex"
    # Must exist, finish within 10s, and exit 0
    # Redirect stderr to suppress crash messages from buggy exercises
    [[ -x "$bin" ]] && timeout 10 "$bin" > /dev/null 2>&1
}
# Suppress segfault messages from the shell itself
ulimit -c 0 2>/dev/null || true

total_exercises=0
total_passed=0

echo -e "${BOLD}╔══════════════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║                C++ Systems Programming Progress                     ║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════════════════════════════╝${RESET}"
echo ""

for entry in "${MODULES[@]}"; do
    IFS=':' read -r dir goal exercises_str <<< "$entry"
    IFS=',' read -r -a exercises <<< "$exercises_str"

    module_num="${dir:7:2}"
    module_label="${dir#module_??_}"

    ex_total=${#exercises[@]}
    ex_passed=0
    ex_results=()

    for ex in "${exercises[@]}"; do
        if check_exercise "$dir" "$ex"; then
            ex_passed=$((ex_passed + 1))
            ex_results+=("${GREEN}✓${RESET}")
        else
            ex_results+=("${RED}✗${RESET}")
        fi
    done

    total_exercises=$((total_exercises + ex_total))
    total_passed=$((total_passed + ex_passed))

    if [ "$ex_passed" -eq "$ex_total" ]; then
        badge="${GREEN}[✓ DONE    ]${RESET}"
    elif [ "$ex_passed" -eq 0 ]; then
        badge="${RED}[✗ 0/$ex_total      ]${RESET}"
    else
        badge="${RED}[✗ $ex_passed/$ex_total      ]${RESET}"
    fi

    printf "${BOLD}%s. %-22s${RESET} %b  %s\n" \
        "$module_num" "$module_label" "$badge" "$goal"

    printf "   "
    for r in "${ex_results[@]}"; do
        printf "%b " "$r"
    done
    printf "\n\n"
done

echo -e "${BOLD}──────────────────────────────────────────────────────────────────────${RESET}"

pct=0
[ "$total_exercises" -gt 0 ] && pct=$(( total_passed * 100 / total_exercises ))

if [ "$total_passed" -eq "$total_exercises" ]; then
    color="$GREEN"
else
    color="$RED"
fi

echo -e "  Total: ${color}${BOLD}${total_passed}/${total_exercises}${RESET} exercises passing (${pct}%)"
echo ""
