// EXERCISE 4: Fix the thread pool shutdown so it doesn't deadlock.
//
// Worker threads block on `cv.wait()` when the task queue is empty.
// When the pool is destroyed, `stop` is set to true — but the workers
// are asleep and never see it.
//
// Fix: after setting `stop = true`, call `cv.notify_all()` to wake every
// sleeping worker so they can check the flag and exit.
//
// Build & run:
//   cmake --build build --target ex04_thread_pool && ./build/module_03_concurrency_patterns/ex04_thread_pool

#include <cstdio>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <functional>
#include <queue>
#include <vector>
#include <atomic>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct ThreadPool {
    std::vector<std::thread>          workers;
    std::queue<std::function<void()>> tasks;
    std::mutex                        mtx;
    std::condition_variable           cv;
    bool                              stop = false;

    explicit ThreadPool(int n) {
        for (int i = 0; i < n; i++) {
            workers.emplace_back([this]{
                while (true) {
                    std::function<void()> task;
                    {
                        std::unique_lock<std::mutex> lk(mtx);
                        cv.wait(lk, [this]{ return stop || !tasks.empty(); });
                        if (stop && tasks.empty()) return;
                        task = std::move(tasks.front());
                        tasks.pop();
                    }
                    task();
                }
            });
        }
    }

    void submit(std::function<void()> f) {
        { std::lock_guard<std::mutex> lk(mtx); tasks.push(std::move(f)); }
        cv.notify_one();
    }

    ~ThreadPool() {
        {
            std::lock_guard<std::mutex> lk(mtx);
            stop = true;
        }
        for (auto& t : workers) t.join();
    }
};

int main() {
    {
        const int N = 100;
        std::atomic<int> counter{0};
        {
            ThreadPool pool(4);
            for (int i = 0; i < N; i++)
                pool.submit([&]{ ++counter; });
        }  // pool destructor joins all workers
        CHECK(counter.load() == N);
    }

    {
        const int N = 50;
        std::atomic<int> sum{0};
        {
            ThreadPool pool(2);
            for (int i = 1; i <= N; i++) {
                int val = i;
                pool.submit([&sum, val]{ sum += val; });
            }
        }
        CHECK(sum.load() == N * (N + 1) / 2);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
