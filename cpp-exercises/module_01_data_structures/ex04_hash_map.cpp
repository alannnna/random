// EXERCISE 4: Implement put() and get() for an open-addressing hash map
// that resolves collisions with linear probing.
//
// The table is a fixed array of Slots (key, value, occupied flag).
//
// put(key, val):
//   1. Hash the key to get a start index.
//   2. Walk forward (wrapping around) until you find an empty slot OR the same key.
//   3. Write key+val and mark the slot occupied. If the key already existed, update in place.
//
// get(key):
//   1. Hash the key to get a start index.
//   2. Walk forward until you find a slot with a matching key (return its value)
//      or an unoccupied slot (return nullopt — the key was never inserted).
//
// Note: this simple version does not support deletion.
//
// Build & run:
//   cmake --build build --target ex04_hash_map && ./build/module_01_data_structures/ex04_hash_map

#include <cstdio>
#include <string>
#include <optional>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct HashMap {
    struct Slot {
        std::string key;
        int  value    = 0;
        bool occupied = false;
    };

    std::vector<Slot> table;
    int capacity;

    explicit HashMap(int cap = 16) : table(cap), capacity(cap) {}

    size_t hash(const std::string& key) const {
        size_t h = 0;
        for (char c : key) h = h * 31 + static_cast<unsigned char>(c);
        return h % static_cast<size_t>(capacity);
    }

    // TODO: implement — insert or update key->val using linear probing
    void put(const std::string& key, int val) {
        // your code here
    }

    // TODO: implement — return value for key, or nullopt if not present
    std::optional<int> get(const std::string& key) const {
        // your code here
        return std::nullopt;
    }
};

int main() {
    // Basic insert and lookup
    {
        HashMap m(8);
        m.put("a", 1);
        m.put("b", 2);
        m.put("c", 3);
        CHECK(m.get("a") == 1);
        CHECK(m.get("b") == 2);
        CHECK(m.get("c") == 3);
        CHECK(m.get("z") == std::nullopt);
    }

    // Update an existing key
    {
        HashMap m(8);
        m.put("x", 10);
        m.put("x", 99);
        CHECK(m.get("x") == 99);
    }

    // Fill most of the table to force probing
    {
        HashMap m(16);
        for (int i = 0; i < 12; i++) m.put(std::to_string(i), i * 10);
        for (int i = 0; i < 12; i++) CHECK(m.get(std::to_string(i)) == i * 10);
        CHECK(m.get("999") == std::nullopt);
    }

    // Keys that are likely to collide (same hash bucket after mod)
    {
        HashMap m(4);  // tiny table — almost everything collides
        m.put("aa", 1);
        m.put("bb", 2);
        m.put("cc", 3);
        CHECK(m.get("aa") == 1);
        CHECK(m.get("bb") == 2);
        CHECK(m.get("cc") == 3);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
