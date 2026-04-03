// EXERCISE 3: Fix the lost-wakeup bug in the consumer.
//
// The broken code calls cv.wait(lk) with NO predicate. This causes a hang:
//
//   1. Consumer pops the last item and calls pop() again.
//   2. Producer finishes, sets done=true, calls cv.notify_all() — but
//      the consumer hasn't called wait() yet, so the notification is lost.
//   3. Consumer calls cv.wait(lk) and sleeps forever — nobody will wake it.
//
// The fix: use the predicate overload, which re-checks the condition
// BEFORE sleeping (so a "past" notification is never missed):
//
//   cv.wait(lk, [&]{ return !q.empty() || done; });
//
// This also handles spurious wakeups: the lambda is re-evaluated on every
// wake-up, so the thread only proceeds when there's real work (or we're done).
//
// Build & run:
//   cmake --build build --target ex03_condition_var && ./build/module_02_threads/ex03_condition_var

#include <cstdio>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <queue>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct SharedQueue {
    std::queue<int>         q;
    std::mutex              mtx;
    std::condition_variable cv;
    bool done = false;

    void push(int val) {
        { std::lock_guard<std::mutex> lk(mtx); q.push(val); }
        cv.notify_one();
    }

    void finish() {
        { std::lock_guard<std::mutex> lk(mtx); done = true; }
        cv.notify_all();
    }

    // Returns true and sets `out` if an item was consumed; false when finished.
    bool pop(int& out) {
        std::unique_lock<std::mutex> lk(mtx);
        // thread reaches wait(), the wakeup is lost and this sleeps forever.
        // Fix: cv.wait(lk, [&]{ return !q.empty() || done; });
        cv.wait(lk);
        if (q.empty()) return false;
        out = q.front(); q.pop();
        return true;
    }
};

int main() {
    const int N = 200;

    for (int trial = 0; trial < 5; trial++) {
        SharedQueue sq;
        std::vector<int> results;
        std::mutex res_mtx;

        std::thread producer([&]{
            for (int i = 0; i < N; i++) sq.push(i);
            sq.finish();
        });

        std::thread consumer([&]{
            int val;
            while (sq.pop(val)) {
                std::lock_guard<std::mutex> lk(res_mtx);
                results.push_back(val);
            }
        });

        producer.join();
        consumer.join();

        CHECK(static_cast<int>(results.size()) == N);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
