// EXERCISE 5: Fix the writer to use std::unique_lock with std::shared_mutex.
//
// std::shared_mutex supports two locking modes:
//   shared  (std::shared_lock)  — multiple readers can hold this simultaneously
//   exclusive (std::unique_lock) — only one writer; no readers allowed at the same time
//
// The write() method below uses shared_lock, so multiple writers run concurrently
// and corrupt `value` with a data race.
//
// Fix: change write()'s lock to std::unique_lock<std::shared_mutex>.
//
// Build & run:
//   cmake --build build --target ex05_rwlock && ./build/module_02_threads/ex05_rwlock

#include <cstdio>
#include <thread>
#include <shared_mutex>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct RWValue {
    int value = 0;
    std::shared_mutex rw_mtx;

    int read() {
        std::shared_lock<std::shared_mutex> lk(rw_mtx);
        return value;
    }

    void write(int delta) {
        std::shared_lock<std::shared_mutex> lk(rw_mtx);
        // Read-yield-write makes concurrent writers extremely likely to overlap,
        // producing a lost-update (final value < N) that's easy to detect.
        int tmp = value;
        std::this_thread::yield();
        value = tmp + delta;
    }
};

int main() {
    const int N = 1000;

    for (int trial = 0; trial < 5; trial++) {
        RWValue rv;
        std::vector<std::thread> writers;
        writers.reserve(N);
        for (int i = 0; i < N; i++)
            writers.emplace_back([&]{ rv.write(1); });
        for (auto& t : writers) t.join();
        CHECK(rv.read() == N);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
