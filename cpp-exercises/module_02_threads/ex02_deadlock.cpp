// EXERCISE 2: Fix the deadlock in transfer().
//
// transfer(A→B) locks A's mutex then B's.
// transfer(B→A) locks B's mutex then A's.
// When both run concurrently each thread holds one lock and waits for the other — deadlock.
//
// Fix: acquire both locks simultaneously using std::lock(), which uses a
// deadlock-avoidance algorithm. Then adopt them into lock_guards:
//
//   std::lock(from.mtx, to.mtx);
//   std::lock_guard<std::mutex> l1(from.mtx, std::adopt_lock);
//   std::lock_guard<std::mutex> l2(to.mtx,   std::adopt_lock);
//
// Build & run:
//   cmake --build build --target ex02_deadlock && ./build/module_02_threads/ex02_deadlock

#include <cstdio>
#include <thread>
#include <mutex>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct Account {
    int balance;
    std::mutex mtx;
    explicit Account(int b) : balance(b) {}
};

void transfer(Account& from, Account& to, int amount) {
    // thread B holds to.mtx and waits for from.mtx → ABBA deadlock.
    // The yield between the two lock() calls makes the interleaving near-certain.
    // Fix: replace with std::scoped_lock(from.mtx, to.mtx)
    //      or:  std::lock(from.mtx, to.mtx);
    //           std::lock_guard l1(from.mtx, std::adopt_lock);
    //           std::lock_guard l2(to.mtx,   std::adopt_lock);
    from.mtx.lock();
    std::this_thread::yield();  // let the other thread grab to.mtx before we do
    to.mtx.lock();
    from.balance -= amount;
    to.balance   += amount;
    to.mtx.unlock();
    from.mtx.unlock();
}

int main() {
    const int N = 500;
    Account a(10000), b(10000);

    std::vector<std::thread> threads;
    threads.reserve(N * 2);
    for (int i = 0; i < N; i++) {
        threads.emplace_back([&]{ transfer(a, b, 1); });
        threads.emplace_back([&]{ transfer(b, a, 1); });
    }
    for (auto& t : threads) t.join();

    CHECK(a.balance == 10000);
    CHECK(b.balance == 10000);
    CHECK(a.balance + b.balance == 20000);

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
