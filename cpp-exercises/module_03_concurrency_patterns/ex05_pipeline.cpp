// EXERCISE 5: Fix the two-stage pipeline so stage 2 doesn't exit early.
//
// Stage 1 produces numbers 0..N-1 into a shared queue.
// Stage 2 reads from that queue and squares each number.
//
// The bug: stage 2 exits as soon as it sees the queue empty, but stage 1
// may not be done yet — stage 2 can drain the queue between two of stage 1's
// pushes and terminate prematurely, dropping items.
//
// Fix: add a `bool stage1_done` flag. Stage 2 should only exit when BOTH
// the queue is empty AND stage1_done is true.
// Use a condition variable (or check the flag inside the pop wait predicate).
//
// Build & run:
//   cmake --build build --target ex05_pipeline && ./build/module_03_concurrency_patterns/ex05_pipeline

#include <cstdio>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <queue>
#include <vector>
#include <numeric>
#include <algorithm>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Pipeline {
    std::queue<int>         q;
    std::mutex              mtx;
    std::condition_variable cv;
    bool                    stage1_done = false;

    std::vector<int> results;

    void run(int n) {
        std::thread stage1([&]{
            for (int i = 0; i < n; i++) {
                // Small pause so stage2 reliably sees the queue empty between pushes
                std::this_thread::sleep_for(std::chrono::microseconds(200));
                { std::lock_guard<std::mutex> lk(mtx); q.push(i); }
                cv.notify_one();
            }
            { std::lock_guard<std::mutex> lk(mtx); stage1_done = true; }
            cv.notify_all();
        });

        std::thread stage2([&]{
            while (true) {
                std::unique_lock<std::mutex> lk(mtx);
                // BUG: no cv.wait — polls and exits the moment the queue is transiently
                // empty, even when stage1 is still producing.
                // Fix: cv.wait(lk, [&]{ return !q.empty() || stage1_done; });
                if (q.empty()) break;
                int val = q.front(); q.pop();
                lk.unlock();
                results.push_back(val * val);
            }
        });

        stage1.join();
        stage2.join();
    }
};

int main() {
    for (int trial = 0; trial < 10; trial++) {
        Pipeline p;
        const int N = 50;
        p.run(N);

        CHECK(static_cast<int>(p.results.size()) == N);

        // Verify the squared values are all present (order may vary)
        std::vector<int> expected;
        for (int i = 0; i < N; i++) expected.push_back(i * i);
        std::sort(p.results.begin(), p.results.end());
        std::sort(expected.begin(), expected.end());
        CHECK(p.results == expected);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
