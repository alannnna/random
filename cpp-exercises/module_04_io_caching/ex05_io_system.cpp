// EXERCISE 5: Implement a BufferedLog — a write-ahead log with flush policies.
//
// BufferedLog accumulates log records in memory and flushes them to `sink`
// (simulated output) based on two policies:
//   - Size policy:  flush when buffer reaches `byte_capacity` bytes
//   - Count policy: flush when `count_capacity` records have been appended
//
// You implement three methods:
//
//   append(record):
//     Add `record` to the buffer and increment record_count.
//     If buffer.size() >= byte_capacity OR record_count >= count_capacity,
//     call flush().
//
//   flush():
//     Move buffer contents into sink (push the current buffer string onto
//     sink, then clear buffer and reset record_count to 0).
//     Do nothing if buffer is empty.
//
//   close():
//     Flush any remaining buffered data (partial batch), then mark closed.
//
// Build & run:
//   cmake --build build --target ex05_io_system && ./build/module_04_io_caching/ex05_io_system

#include <cstdio>
#include <string>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

struct BufferedLog {
    std::vector<std::string> sink;          // simulates durable storage (append-only)
    std::string              buffer;        // in-memory accumulation buffer
    int                      record_count = 0;
    const size_t             byte_capacity;
    const int                count_capacity;
    bool                     closed = false;

    BufferedLog(size_t byte_cap, int count_cap)
        : byte_capacity(byte_cap), count_capacity(count_cap) {}

    // TODO: add record to buffer, flush if either capacity threshold is reached
    void append(const std::string& record) {
        // your code here
    }

    // TODO: push buffer to sink, clear buffer, reset record_count
    void flush() {
        // your code here
    }

    // TODO: flush any remaining data and mark closed
    void close() {
        // your code here
    }

    std::string all_data() const {
        std::string out;
        for (auto& s : sink) out += s;
        out += buffer;  // include un-flushed data for inspection
        return out;
    }
};

int main() {
    // Byte-capacity flush: each chunk in sink should be <= byte_capacity
    {
        BufferedLog log(10, 1000);
        std::string all;
        for (int i = 0; i < 15; i++) {
            std::string r = "rec" + std::to_string(i) + ";";
            all += r;
            log.append(r);
        }
        log.close();

        CHECK(log.all_data() == all);
        for (auto& chunk : log.sink)
            CHECK(chunk.size() <= log.byte_capacity);
    }

    // Count-capacity flush: flush every 3 records
    {
        BufferedLog log(10000, 3);
        for (int i = 0; i < 9; i++) log.append("x");
        // 9 records / 3 per flush = 3 flushes
        CHECK(static_cast<int>(log.sink.size()) == 3);
        log.close();
    }

    // close() flushes partial batch
    {
        BufferedLog log(1000, 1000);
        log.append("hello");
        log.append("world");
        CHECK(log.sink.empty());   // not flushed yet
        log.close();
        CHECK(!log.sink.empty());  // close forced flush
        CHECK(log.all_data() == "helloworld");
    }

    // flush() on empty buffer does nothing
    {
        BufferedLog log(10, 5);
        log.flush();
        CHECK(log.sink.empty());
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
