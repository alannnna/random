foreach(ex
    ex01_linked_list
    ex02_stack
    ex03_ring_buffer
    ex04_hash_map
    ex05_lru_cache
)
    add_executable(${ex} ${ex}.cpp)
    target_link_libraries(${ex} PRIVATE Threads::Threads)
endforeach()
