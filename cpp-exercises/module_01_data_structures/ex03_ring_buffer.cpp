// EXERCISE 3: Fix is_empty() and is_full() in the ring buffer.
//
// This ring buffer uses the "sacrifice one slot" convention to distinguish
// empty from full without a separate size counter:
//
//   empty: tail == head
//   full:  (tail + 1) % capacity == head
//
// The two predicates below are swapped — fix them.
//
// Build & run:
//   cmake --build build --target ex03_ring_buffer && ./build/module_01_data_structures/ex03_ring_buffer

#include <cstdio>
#include <vector>
#include <optional>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct RingBuffer {
    std::vector<int> buf;
    int head = 0;   // read index
    int tail = 0;   // write index
    int capacity;

    explicit RingBuffer(int cap) : buf(cap), capacity(cap) {}

    bool is_empty() const { return (tail + 1) % capacity == head; }  // BUG: this is actually is_full
    bool is_full()  const { return tail == head; }                     // BUG: this is actually is_empty

    bool push(int val) {
        if (is_full()) return false;
        buf[tail] = val;
        tail = (tail + 1) % capacity;
        return true;
    }

    std::optional<int> pop() {
        if (is_empty()) return std::nullopt;
        int val = buf[head];
        head = (head + 1) % capacity;
        return val;
    }
};

int main() {
    // capacity=4 means 3 usable slots (one sacrificed)
    RingBuffer rb(4);

    CHECK(rb.is_empty());
    CHECK(!rb.is_full());

    CHECK(rb.push(10) == true);
    CHECK(rb.push(20) == true);
    CHECK(rb.push(30) == true);
    CHECK(rb.push(40) == false);  // 4th push must fail — buffer is full
    CHECK(rb.is_full());
    CHECK(!rb.is_empty());

    CHECK(rb.pop() == 10);
    CHECK(rb.pop() == 20);
    CHECK(!rb.is_empty());

    CHECK(rb.push(40) == true);   // room again after popping two

    CHECK(rb.pop() == 30);
    CHECK(rb.pop() == 40);
    CHECK(rb.pop() == std::nullopt);  // empty
    CHECK(rb.is_empty());

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
