// EXERCISE 2: Fix read_lines() to include the final line when there is no
// trailing newline.
//
// Most real files don't end with '\n'. The current implementation discards
// whatever is left in `line` after the loop — the last (or only) line is lost.
//
// Fix: after the loop, if `line` is non-empty, push it into `lines`.
//
// Build & run:
//   cmake --build build --target ex02_line_reader && ./build/module_04_io_caching/ex02_line_reader

#include <cstdio>
#include <string>
#include <vector>

static int _pass = 0, _fail = 0;
#define CHECK(expr) do { if(expr){++_pass;}else{++_fail; \
    fprintf(stderr,"FAIL [line %d]: %s\n",__LINE__,#expr);} } while(0)

std::vector<std::string> read_lines(const std::string& content) {
    std::vector<std::string> lines;
    std::string line;
    for (char c : content) {
        if (c == '\n') {
            lines.push_back(line);
            line.clear();
        } else {
            line += c;
        }
    }
    // BUG: missing — if (!line.empty()) lines.push_back(line);
    return lines;
}

int main() {
    // No trailing newline — last line must be included
    {
        auto lines = read_lines("hello\nworld");
        CHECK(lines.size() == 2);
        CHECK(lines[0] == "hello");
        CHECK(lines[1] == "world");
    }

    // With trailing newline — behaviour unchanged
    {
        auto lines = read_lines("hello\nworld\n");
        CHECK(lines.size() == 2);
        CHECK(lines[0] == "hello");
        CHECK(lines[1] == "world");
    }

    // Single line, no newline
    {
        auto lines = read_lines("onlyone");
        CHECK(lines.size() == 1);
        CHECK(lines[0] == "onlyone");
    }

    // Empty string
    {
        auto lines = read_lines("");
        CHECK(lines.size() == 0);
    }

    // Only a newline
    {
        auto lines = read_lines("\n");
        CHECK(lines.size() == 1);
        CHECK(lines[0] == "");
    }

    // Multiple blank lines — use whole-vector comparison to avoid out-of-bounds
    // if the buggy implementation returns fewer elements than expected
    {
        auto lines = read_lines("a\n\nb\n\nc");
        std::vector<std::string> expected = {"a", "", "b", "", "c"};
        CHECK(lines == expected);
    }

    if (_fail > 0) { fprintf(stderr, "%d/%d tests failed\n", _fail, _pass+_fail); return 1; }
    printf("%d tests passed\n", _pass);
    return 0;
}
