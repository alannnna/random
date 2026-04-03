// EXERCISE 3: Fix put() in the write-through cache.
//
// A write-through cache keeps a fast in-memory cache AND a slower backing store
// (here simulated by a map) in sync. Every write must update BOTH.
//
// The bug: put() writes to the cache but silently skips the backing store.
// After a cache eviction, the data is lost.
//
// Fix: add `backing_store[key] = val;` inside put().
//
// Build & run:
//   cmake --build build --target ex03_write_through_cache && ./build/module_04_io_caching/ex03_write_through_cache

#include <cstdio>
#include <string>
#include <unordered_map>
#include <optional>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct WriteThroughCache {
    std::unordered_map<std::string, int> cache;
    std::unordered_map<std::string, int> backing_store;  // simulates persistent storage

    void put(const std::string& key, int val) {
        cache[key] = val;
    }

    // Simulate eviction: remove key from cache (backing store unaffected)
    void evict(const std::string& key) {
        cache.erase(key);
    }

    // Read: check cache first, fall back to backing store (read-through)
    std::optional<int> get(const std::string& key) const {
        auto it = cache.find(key);
        if (it != cache.end()) return it->second;
        auto it2 = backing_store.find(key);
        if (it2 != backing_store.end()) return it2->second;
        return std::nullopt;
    }
};

int main() {
    // Data survives cache eviction only if backing store was updated
    {
        WriteThroughCache c;
        c.put("x", 42);
        c.put("y", 99);
        c.evict("x");
        CHECK(c.get("x") == 42);   // must come from backing store
        CHECK(c.get("y") == 99);   // still in cache
    }

    // Update propagates to backing store
    {
        WriteThroughCache c;
        c.put("k", 1);
        c.put("k", 2);   // update
        c.evict("k");
        CHECK(c.get("k") == 2);    // backing store should have latest value
    }

    // Missing key returns nullopt
    {
        WriteThroughCache c;
        CHECK(c.get("nope") == std::nullopt);
    }

    // Multiple keys
    {
        WriteThroughCache c;
        for (int i = 0; i < 10; i++) c.put(std::to_string(i), i * 10);
        for (int i = 0; i < 10; i++) c.evict(std::to_string(i));
        for (int i = 0; i < 10; i++) CHECK(c.get(std::to_string(i)) == i * 10);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
