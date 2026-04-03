// EXERCISE 4: Fix the reference counter using std::atomic<int>.
//
// The buggy version uses a plain `int` and explicit load-yield-store operations.
// The yield forces interleaving: two threads both read the same old count,
// increment their local copy, then both write back the same new value — losing one update.
//
// Two-step fix:
//   1. Change `int count` to `std::atomic<int> count`
//   2. Replace the explicit load/store in each method with a single atomic operation:
//        add_ref()  →  count.fetch_add(1)
//        release()  →  return count.fetch_sub(1) == 1
//        use_count() →  return count.load()
//
// fetch_add/fetch_sub are indivisible read-modify-write operations — no separate
// load and store that another thread can slip between.
//
// Build & run:
//   cmake --build build --target ex04_atomics && ./build/module_02_threads/ex04_atomics

#include <cstdio>
#include <thread>
#include <atomic>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct RefCounter {
    int count = 0;

    // Explicit load + yield + store guarantees the race:
    // thread A reads count=5, thread B reads count=5, both write 6 — one increment lost.
    // Fix: use count.fetch_add(1) and count.fetch_sub(1) — single indivisible operations.
    void add_ref() {
        int tmp = count;
        std::this_thread::yield();
        count = tmp + 1;
    }
    bool release() {
        int tmp = count;
        std::this_thread::yield();
        count = tmp - 1;
        return count == 0;
    }
    int use_count() const { return count; }
};

int main() {
    const int N = 2000;

    for (int trial = 0; trial < 5; trial++) {
        RefCounter rc;
        std::vector<std::thread> threads;
        threads.reserve(N);

        for (int i = 0; i < N; i++)
            threads.emplace_back([&]{ rc.add_ref(); });
        for (auto& t : threads) t.join();
        CHECK(rc.use_count() == N);

        threads.clear();
        for (int i = 0; i < N; i++)
            threads.emplace_back([&]{ rc.release(); });
        for (auto& t : threads) t.join();
        CHECK(rc.use_count() == 0);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
