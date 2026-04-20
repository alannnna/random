foreach(ex
    ex01_vector_add
    ex02_missing_guard
    ex03_shared_sync
    ex04_atomic_counter
    ex05_unified_sync
)
    add_executable(${ex} ${ex}.cu)
endforeach()
