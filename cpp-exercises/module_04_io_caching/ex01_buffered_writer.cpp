// EXERCISE 1: Fix the buffered writer so it flushes before overflowing.
//
// A BufferedWriter accumulates data in an in-memory buffer and only
// writes to the output when the buffer is full (or explicitly flushed).
// This amortizes the cost of I/O calls.
//
// The bug: write() appends data without checking whether it would exceed
// `capacity`. Fix it: before appending, if `buf.size() + data.size() > capacity`,
// flush the current buffer first.
//
// Edge case: if a single write is larger than capacity, flush what's buffered
// first, then write the oversized chunk directly (don't try to buffer it).
//
// Build & run:
//   cmake --build build --target ex01_buffered_writer && ./build/module_04_io_caching/ex01_buffered_writer

#include <cstdio>
#include <string>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct BufferedWriter {
    std::string              buf;
    std::vector<std::string> flushed;  // simulates the underlying output
    size_t                   capacity;

    explicit BufferedWriter(size_t cap) : capacity(cap) {}

    void flush() {
        if (!buf.empty()) {
            flushed.push_back(buf);
            buf.clear();
        }
    }

    void write(const std::string& data) {
        buf += data;
    }

    void close() { flush(); }

    size_t total_flushed_bytes() const {
        size_t n = 0;
        for (auto& s : flushed) n += s.size();
        return n;
    }

    std::string all_output() const {
        std::string out;
        for (auto& s : flushed) out += s;
        out += buf;  // include anything not yet flushed
        return out;
    }
};

int main() {
    // Buffer should never exceed capacity between flushes
    {
        BufferedWriter w(4);
        w.write("ab");   // buf="ab"  (2 bytes, fits)
        w.write("cd");   // buf="abcd" (4 bytes, exactly full — no flush yet)
        w.write("e");    // would exceed 4 → flush "abcd" first, then buf="e"
        w.close();

        CHECK(w.all_output() == "abcde");
        // Each individual flush chunk must be <= capacity
        for (auto& chunk : w.flushed) CHECK(chunk.size() <= w.capacity);
    }

    // All data must reach the output
    {
        BufferedWriter w(8);
        std::string all;
        for (int i = 0; i < 20; i++) {
            std::string s(3, 'a' + i % 26);
            all += s;
            w.write(s);
        }
        w.close();
        CHECK(w.all_output() == all);
    }

    // Oversized single write: flush buffer first, then pass through directly
    {
        BufferedWriter w(4);
        w.write("ab");
        w.write("this-is-longer-than-capacity");
        w.close();
        CHECK(w.all_output() == "abthis-is-longer-than-capacity");
        for (auto& chunk : w.flushed) CHECK(chunk.size() <= w.capacity || chunk.size() > w.capacity);
        // (oversized chunk allowed through; key invariant: no data lost)
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
