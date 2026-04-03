// EXERCISE 1: Fix the data race on the shared counter.
//
// Ten threads each call increment() 1000 times. Without synchronization,
// concurrent read-modify-write on `value` loses updates and the final
// count is unpredictable.
//
// Fix: acquire the mutex inside increment() before touching `value`.
// Use std::lock_guard<std::mutex> — it releases automatically on scope exit.
//
// Build & run:
//   cmake --build build --target ex01_data_race && ./build/module_02_threads/ex01_data_race

#include <cstdio>
#include <thread>
#include <mutex>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Counter {
    int value = 0;
    std::mutex mtx;  // hint: use this

    void increment() {
        // two threads both read the same value, then both write value+1, losing one update.
        int tmp = value;
        std::this_thread::yield();
        value = tmp + 1;
    }
};

int main() {
    const int N_THREADS = 10, N_ITERS = 1000;

    for (int trial = 0; trial < 3; trial++) {
        Counter c;
        std::vector<std::thread> threads;
        threads.reserve(N_THREADS);
        for (int i = 0; i < N_THREADS; i++)
            threads.emplace_back([&]{ for (int j = 0; j < N_ITERS; j++) c.increment(); });
        for (auto& t : threads) t.join();
        CHECK(c.value == N_THREADS * N_ITERS);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
