foreach(ex
    ex01_bounded_queue
    ex02_semaphore
    ex03_barrier
    ex04_thread_pool
    ex05_pipeline
)
    add_executable(${ex} ${ex}.cpp)
    target_link_libraries(${ex} PRIVATE Threads::Threads)
endforeach()
