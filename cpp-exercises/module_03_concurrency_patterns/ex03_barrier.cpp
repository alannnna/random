// EXERCISE 3: Implement wait() for a reusable thread barrier.
//
// A barrier lets N threads rendezvous: each thread calls wait() and blocks
// until all N have arrived, then all proceed together.
// "Reusable" means it resets automatically and can be used for multiple rounds.
//
// The standard pattern uses a generation counter to avoid the ABA problem:
// after the last thread arrives and resets `count`, waiting threads must not
// accidentally re-enter the wait for the next round.
//
//   void wait() {
//       std::unique_lock<std::mutex> lk(mtx);
//       int my_gen = generation;
//       if (++count == total) {
//           count = 0;
//           ++generation;             // advance epoch so old waiters wake correctly
//           cv.notify_all();
//       } else {
//           cv.wait(lk, [&]{ return generation != my_gen; });
//       }
//   }
//
// Build & run:
//   cmake --build build --target ex03_barrier && ./build/module_03_concurrency_patterns/ex03_barrier

#include <cstdio>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <atomic>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Barrier {
    std::mutex              mtx;
    std::condition_variable cv;
    int count      = 0;
    int generation = 0;
    const int total;

    explicit Barrier(int n) : total(n) {}

    // TODO: implement reusable barrier wait (see instructions above)
    void wait() {
        // your code here
    }
};

int main() {
    // All threads must reach the barrier before any proceeds to phase 2
    {
        const int N = 8;
        Barrier b(N);
        std::atomic<int> phase1_done{0};
        std::atomic<int> phase2_start{0};
        std::vector<std::thread> threads;
        threads.reserve(N);

        for (int i = 0; i < N; i++) {
            threads.emplace_back([&]{
                ++phase1_done;
                b.wait();
                // Every thread should see phase1_done == N here
                if (phase1_done.load() == N) ++phase2_start;
            });
        }
        for (auto& t : threads) t.join();
        CHECK(phase2_start.load() == N);
    }

    // Reusability: barrier works for two rounds
    {
        const int N = 4;
        Barrier b(N);
        std::atomic<int> round1{0}, round2{0};
        std::vector<std::thread> threads;
        threads.reserve(N);

        for (int i = 0; i < N; i++) {
            threads.emplace_back([&]{
                ++round1;
                b.wait();   // round 1
                ++round2;
                b.wait();   // round 2
            });
        }
        for (auto& t : threads) t.join();
        CHECK(round1.load() == N);
        CHECK(round2.load() == N);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
