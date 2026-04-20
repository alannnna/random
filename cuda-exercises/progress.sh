#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"

GREEN='\033[0;32m'
RED='\033[0;31m'
BOLD='\033[1m'
RESET='\033[0m'

declare -a MODULES=(
    "module_01_memory:I understand GPU memory correctness pitfalls:ex01_vector_add,ex02_missing_guard,ex03_shared_sync,ex04_atomic_counter,ex05_unified_sync"
    "module_02_parallel_patterns:I can implement correct parallel algorithms on the GPU:ex01_reduction,ex02_histogram,ex03_prefix_scan,ex04_stencil,ex05_dot_product"
    "module_03_streams:I can use CUDA streams and events for async execution:ex01_async_result,ex02_event_timing,ex03_stream_dependency,ex04_double_buffer,ex05_multi_stream_sync"
    "module_04_warp_optimization:I can write warp-efficient kernels and avoid common pitfalls:ex01_transpose,ex02_tiled_matmul,ex03_warp_reduce,ex04_grid_stride,ex05_warp_sync"
)

mkdir -p "$BUILD_DIR"

echo ""
echo -e "${BOLD}Building...${RESET}"
cmake -S "$SCRIPT_DIR" -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE=Release -Wno-dev \
    > "$BUILD_DIR/cmake_configure.log" 2>&1 || true
cmake --build "$BUILD_DIR" --parallel \
    > "$BUILD_DIR/cmake_build.log" 2>&1 || true
echo ""

check_exercise() {
    local dir="$1"
    local ex="$2"
    local bin="$BUILD_DIR/$dir/$ex"
    [[ -x "$bin" ]] && timeout 15 "$bin" > /dev/null 2>&1
}
ulimit -c 0 2>/dev/null || true

total_exercises=0
total_passed=0

echo -e "${BOLD}╔══════════════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║                    CUDA Programming Progress                        ║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════════════════════════════╝${RESET}"
echo ""

for entry in "${MODULES[@]}"; do
    IFS=':' read -r dir goal exercises_str <<< "$entry"
    IFS=',' read -r -a exercises <<< "$exercises_str"

    module_num="${dir:7:2}"
    module_label="${dir#module_??_}"

    ex_total=${#exercises[@]}
    ex_passed=0
    ex_results=()

    for ex in "${exercises[@]}"; do
        if check_exercise "$dir" "$ex"; then
            ex_passed=$((ex_passed + 1))
            ex_results+=("${GREEN}✓${RESET}")
        else
            ex_results+=("${RED}✗${RESET}")
        fi
    done

    total_exercises=$((total_exercises + ex_total))
    total_passed=$((total_passed + ex_passed))

    if [ "$ex_passed" -eq "$ex_total" ]; then
        badge="${GREEN}[✓ DONE    ]${RESET}"
    elif [ "$ex_passed" -eq 0 ]; then
        badge="${RED}[✗ 0/$ex_total      ]${RESET}"
    else
        badge="${RED}[✗ $ex_passed/$ex_total      ]${RESET}"
    fi

    printf "${BOLD}%s. %-22s${RESET} %b  %s\n" \
        "$module_num" "$module_label" "$badge" "$goal"

    printf "   "
    for r in "${ex_results[@]}"; do
        printf "%b " "$r"
    done
    printf "\n\n"
done

echo -e "${BOLD}──────────────────────────────────────────────────────────────────────${RESET}"

pct=0
[ "$total_exercises" -gt 0 ] && pct=$(( total_passed * 100 / total_exercises ))

if [ "$total_passed" -eq "$total_exercises" ]; then
    color="$GREEN"
else
    color="$RED"
fi

echo -e "  Total: ${color}${BOLD}${total_passed}/${total_exercises}${RESET} exercises passing (${pct}%)"
echo ""
