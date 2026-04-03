// EXERCISE 1: Fix push() in the bounded blocking queue.
//
// This queue should block the producer when full and block the consumer when empty.
// The bug: push() never checks whether the queue is at capacity — it just inserts,
// potentially letting the queue grow without bound.
//
// Fix: before inserting, wait on `not_full` while `q.size() == max_size`.
//
//   not_full.wait(lk, [&]{ return q.size() < max_size; });
//
// Build & run:
//   cmake --build build --target ex01_bounded_queue && ./build/module_03_concurrency_patterns/ex01_bounded_queue

#include <cstdio>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <queue>
#include <vector>
#include <atomic>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct BoundedQueue {
    std::queue<int>         q;
    std::mutex              mtx;
    std::condition_variable not_empty;
    std::condition_variable not_full;
    const size_t            max_size;

    explicit BoundedQueue(size_t cap) : max_size(cap) {}

    void push(int val) {
        std::unique_lock<std::mutex> lk(mtx);
        q.push(val);
        not_empty.notify_one();
    }

    int pop() {
        std::unique_lock<std::mutex> lk(mtx);
        not_empty.wait(lk, [&]{ return !q.empty(); });
        int val = q.front(); q.pop();
        not_full.notify_one();
        return val;
    }

    size_t size() {
        std::lock_guard<std::mutex> lk(mtx);
        return q.size();
    }
};

int main() {
    // Verify the queue never exceeds capacity under concurrent push/pop
    {
        const size_t CAP = 4;
        const int    N   = 200;
        BoundedQueue bq(CAP);
        std::atomic<int> max_observed{0};

        std::thread producer([&]{
            for (int i = 0; i < N; i++) {
                bq.push(i);
                int s = static_cast<int>(bq.size());
                int prev = max_observed.load();
                while (s > prev && !max_observed.compare_exchange_weak(prev, s)) {}
            }
        });

        std::thread consumer([&]{
            for (int i = 0; i < N; i++) bq.pop();
        });

        producer.join();
        consumer.join();

        CHECK(max_observed.load() <= static_cast<int>(CAP));
        CHECK(bq.size() == 0);
    }

    // Correctness: all pushed items must be popped
    {
        BoundedQueue bq(3);
        const int N = 100;
        std::vector<int> got;
        std::mutex got_mtx;

        std::thread prod([&]{ for (int i = 0; i < N; i++) bq.push(i); });
        std::thread cons([&]{
            for (int i = 0; i < N; i++) {
                std::lock_guard<std::mutex> lk(got_mtx);
                got.push_back(bq.pop());
            }
        });
        prod.join(); cons.join();

        CHECK(static_cast<int>(got.size()) == N);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
