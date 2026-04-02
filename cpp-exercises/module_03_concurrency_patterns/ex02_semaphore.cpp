// EXERCISE 2: Implement acquire() and release() for a counting semaphore.
//
// A counting semaphore controls access to a pool of N resources.
//
//   acquire(): decrement the count, blocking if it would go below 0 —
//              wait until another thread releases a resource
//
//   release(): increment the count and wake one waiting thread
//
// The struct has a mutex, condition variable, and int count already declared.
//
// Key pattern for acquire():
//   std::unique_lock<std::mutex> lk(mtx);
//   cv.wait(lk, [&]{ return count > 0; });   // loop-safe wait with predicate
//   --count;
//
// Key pattern for release():
//   { std::lock_guard<std::mutex> lk(mtx); ++count; }
//   cv.notify_one();
//
// Build & run:
//   cmake --build build --target ex02_semaphore && ./build/module_03_concurrency_patterns/ex02_semaphore

#include <cstdio>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <atomic>
#include <chrono>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Semaphore {
    std::mutex              mtx;
    std::condition_variable cv;
    int count;

    explicit Semaphore(int initial) : count(initial) {}

    // TODO: block until count > 0, then decrement
    void acquire() {
        // your code here
    }

    // TODO: increment count and wake one waiting thread
    void release() {
        // your code here
    }
};

int main() {
    // Only `permits` threads should be inside the critical section at once
    {
        const int permits = 3;
        const int workers = 12;
        Semaphore sem(permits);
        std::atomic<int> inside{0};
        std::atomic<int> max_inside{0};
        std::vector<std::thread> threads;
        threads.reserve(workers);

        for (int i = 0; i < workers; i++) {
            threads.emplace_back([&]{
                sem.acquire();
                int cur = ++inside;
                // record high-water mark
                int prev = max_inside.load();
                while (cur > prev && !max_inside.compare_exchange_weak(prev, cur)) {}
                std::this_thread::sleep_for(std::chrono::milliseconds(5));
                --inside;
                sem.release();
            });
        }
        for (auto& t : threads) t.join();

        CHECK(max_inside.load() <= permits);
        CHECK(inside.load() == 0);
    }

    // Semaphore with initial count 0: release then acquire
    {
        Semaphore sem(0);
        std::atomic<bool> acquired{false};

        std::thread waiter([&]{
            sem.acquire();
            acquired = true;
        });

        std::this_thread::sleep_for(std::chrono::milliseconds(20));
        CHECK(acquired.load() == false);  // still blocked
        sem.release();
        waiter.join();
        CHECK(acquired.load() == true);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
