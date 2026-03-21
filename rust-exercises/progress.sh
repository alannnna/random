#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
GRAY='\033[0;37m'
BOLD='\033[1m'
DIM='\033[2m'
RESET='\033[0m'

# Module definitions: "package:goal:ex1,ex2,..."
declare -a MODULES=(
    "module_01_greeter:I can write and run a Rust program:ex01_hello,ex02_mutability,ex03_types,ex04_functions,ex05_final"
    "module_02_guesser:I can write programs with logic and loops:ex01_comparison,ex02_loop,ex03_match,ex04_range,ex05_final"
    "module_03_ownership:I understand who owns data and what happens when it moves:ex01_use_after_move,ex02_clone,ex03_copy,ex04_borrow_not_own,ex05_mut_ownership,ex06_move_in_loop,ex07_struct_method,ex08_final"
    "module_04_borrowing:I can lend data to functions without giving it away:ex01_dangling,ex02_double_mut,ex03_mut_immut_conflict,ex04_move_borrowed,ex05_mut_self,ex06_str_vs_string,ex07_slice_signature,ex08_final"
    "module_05_todos:I can model real problems with custom types:ex01_struct_init,ex02_method_self,ex03_enum_match,ex04_option_unwrap,ex05_result_question"
    "module_06_frequencies:I can process and transform collections of data:ex01_vec_bounds,ex02_hashmap_entry,ex03_iterator_chain,ex04_collect_type,ex05_final"
    "module_07_creatures:I can write reusable code that works across different types:ex01_display,ex02_trait_bound,ex03_default,ex04_from,ex05_iterator"
    "module_08_robust:I can write programs that handle failure without panicking:ex01_panic_to_result,ex02_custom_error,ex03_question_mark,ex04_new_variant,ex05_final"
    "module_09_lifetimes:I can tell the compiler how long references are valid:ex01_two_refs,ex02_explicit_lifetime,ex03_independent_lifetimes,ex04_struct_ref,ex05_impl_lifetime,ex06_self_vs_arg,ex07_elision,ex08_static,ex09_final"
    "module_10_zero_copy:I can write code that is both safe and fast:ex01_key_struct,ex02_config_struct,ex03_parser_fn,ex04_static_inferred,ex05_two_lifetimes,ex06_sub_borrow,ex07_owned_vs_borrowed,ex08_final"
)

check_exercise() {
    local pkg="$1"
    local bin="$2"
    cargo test --bin "$bin" -p "$pkg" --quiet > /dev/null 2>&1
}

total_exercises=0
total_passed=0

echo ""
echo -e "${BOLD}╔══════════════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║                     Rust Learning Progress                          ║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════════════════════════════╝${RESET}"
echo ""

for entry in "${MODULES[@]}"; do
    IFS=':' read -r pkg goal exercises_str <<< "$entry"
    IFS=',' read -r -a exercises <<< "$exercises_str"

    module_num="${pkg:7:2}"  # extract "01" from "module_01_greeter"
    module_label="${pkg#module_??_}"  # extract "greeter" from "module_01_greeter"

    ex_total=${#exercises[@]}
    ex_passed=0
    ex_results=()

    for ex in "${exercises[@]}"; do
        if check_exercise "$pkg" "$ex"; then
            ex_passed=$((ex_passed + 1))
            ex_results+=("${GREEN}✓${RESET}")
        else
            ex_results+=("${RED}✗${RESET}")
        fi
    done

    total_exercises=$((total_exercises + ex_total))
    total_passed=$((total_passed + ex_passed))

    # Status badge
    if [ "$ex_passed" -eq "$ex_total" ]; then
        badge="${GREEN}[✓ DONE    ]${RESET}"
    elif [ "$ex_passed" -eq 0 ]; then
        badge="${RED}[✗ 0/$ex_total      ]${RESET}"
    else
        badge="${RED}[✗ $ex_passed/$ex_total      ]${RESET}"
    fi

    printf "${BOLD}%s. %-14s${RESET} %b  %s\n" \
        "$module_num" "$module_label" "$badge" "$goal"

    # Show individual exercise dots
    printf "   "
    for r in "${ex_results[@]}"; do
        printf "%b " "$r"
    done
    printf "\n\n"
done

echo -e "${BOLD}──────────────────────────────────────────────────────────────────────${RESET}"

pct=0
if [ "$total_exercises" -gt 0 ]; then
    pct=$(( total_passed * 100 / total_exercises ))
fi

if [ "$total_passed" -eq "$total_exercises" ]; then
    color="$GREEN"
else
    color="$RED"
fi

echo -e "  Total: ${color}${BOLD}${total_passed}/${total_exercises}${RESET} exercises passing (${pct}%)"
echo ""
