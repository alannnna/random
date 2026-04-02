foreach(ex
    ex01_buffered_writer
    ex02_line_reader
    ex03_write_through_cache
    ex04_lru_cache_threadsafe
    ex05_io_system
)
    add_executable(${ex} ${ex}.cpp)
    target_link_libraries(${ex} PRIVATE Threads::Threads)
endforeach()
