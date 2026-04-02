// EXERCISE 4: Fix the thread-safe LRU cache — put() is missing its mutex lock.
//
// get() correctly holds the lock for its entire operation.
// put() updates the map and list without holding any lock, so concurrent
// puts can corrupt the data structure (iterator invalidation, double-free, etc.).
//
// Fix: acquire the mutex at the start of put(), just like get() does.
//
// The LRU logic itself is correct — your only change is adding the lock.
//
// Build & run:
//   cmake --build build --target ex04_lru_cache_threadsafe && ./build/module_04_io_caching/ex04_lru_cache_threadsafe

#include <cstdio>
#include <thread>
#include <mutex>
#include <unordered_map>
#include <list>
#include <optional>
#include <vector>
#include <atomic>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct ThreadSafeLRU {
    using KV   = std::pair<int,int>;
    using Iter = std::list<KV>::iterator;

    std::list<KV>               lru;   // front = MRU, back = LRU
    std::unordered_map<int,Iter> map;
    std::mutex                   mtx;
    int                          capacity;

    explicit ThreadSafeLRU(int cap) : capacity(cap) {}

    // Call only after all threads have joined — checks structural invariants.
    // lru.size() must equal map.size() (every list entry has a map entry and vice versa)
    // and size must never exceed capacity.
    bool consistent() const {
        return lru.size() == map.size() &&
               static_cast<int>(lru.size()) <= capacity;
    }

    std::optional<int> get(int key) {
        std::lock_guard<std::mutex> lk(mtx);
        auto it = map.find(key);
        if (it == map.end()) return std::nullopt;
        lru.splice(lru.begin(), lru, it->second);  // move to front
        return it->second->second;
    }

    void put(int key, int val) {
        // BUG: missing lock — concurrent puts corrupt the list/map
        auto it = map.find(key);
        if (it != map.end()) {
            it->second->second = val;
            lru.splice(lru.begin(), lru, it->second);
            return;
        }
        if (static_cast<int>(lru.size()) == capacity) {
            map.erase(lru.back().first);
            lru.pop_back();
        }
        lru.push_front({key, val});
        map[key] = lru.begin();
    }
};

int main() {
    // Structural invariant: after concurrent puts, list_size == map_size <= capacity.
    // Without a lock, two threads can both evict the same tail node simultaneously:
    // - both call map.erase(lru.back().first) — map loses 1 entry
    // - both call lru.pop_back()              — list loses 2 entries
    // This breaks the lru.size() == map.size() invariant, caught by consistent().
    {
        const int THREADS = 200;
        for (int trial = 0; trial < 5; trial++) {
            ThreadSafeLRU cache(5);   // tiny — almost every put triggers an eviction
            std::vector<std::thread> threads;
            threads.reserve(THREADS);
            for (int i = 0; i < THREADS; i++)
                threads.emplace_back([&, i]{ cache.put(i % 15, i); });
            for (auto& t : threads) t.join();
            CHECK(cache.consistent());
        }
    }

    // Basic LRU eviction still works
    {
        ThreadSafeLRU cache(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(1);        // 1 is MRU
        cache.put(3, 30);    // evicts 2
        CHECK(cache.get(2) == std::nullopt);
        CHECK(cache.get(1) == 10);
        CHECK(cache.get(3) == 30);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
