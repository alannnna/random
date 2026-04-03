// EXERCISE 2: Fix MinStack so get_min() returns the correct minimum in O(1).
//
// The auxiliary `mins` vector should track the running minimum at each stack level:
//   mins[i] = min(data[0], data[1], ..., data[i])
// When pushing, store min(val, mins.back()) — not just val.
//
// With the bug, mins is just a copy of data, so after popping the current minimum
// get_min() returns the wrong value.
//
// Build & run:
//   cmake --build build --target ex02_stack && ./build/module_01_data_structures/ex02_stack

#include <cstdio>
#include <vector>
#include <algorithm>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct MinStack {
    std::vector<int> data;
    std::vector<int> mins;  // mins[i] = minimum of data[0..i]

    void push(int val) {
        data.push_back(val);
        mins.push_back(val);
    }

    void pop() {
        data.pop_back();
        mins.pop_back();
    }

    int top()     const { return data.back(); }
    int get_min() const { return mins.back(); }
    bool empty()  const { return data.empty(); }
};

int main() {
    {
        MinStack s;
        s.push(3);
        CHECK(s.get_min() == 3);
        s.push(5);
        CHECK(s.get_min() == 3);  // 5 > 3, min unchanged
        s.push(1);
        CHECK(s.get_min() == 1);  // new minimum
        s.push(2);
        CHECK(s.get_min() == 1);  // 2 > 1, min unchanged
        s.pop();                  // remove 2
        CHECK(s.get_min() == 1);  // 1 still on stack
        s.pop();                  // remove 1
        CHECK(s.get_min() == 3);  // back to 3
    }
    {
        MinStack s;
        s.push(5); s.push(3); s.push(7); s.push(3);
        CHECK(s.get_min() == 3);
        s.pop(); s.pop();          // remove both 3 and 7
        CHECK(s.get_min() == 3);   // one 3 remains
        s.pop();                   // remove remaining 3
        CHECK(s.get_min() == 5);
    }
    {
        MinStack s;
        s.push(10); s.push(20); s.push(5); s.push(15);
        CHECK(s.get_min() == 5);
        s.pop(); s.pop();          // remove 15 and 5
        CHECK(s.get_min() == 10);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
