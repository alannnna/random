foreach(ex
    ex01_async_result
    ex02_event_timing
    ex03_stream_dependency
    ex04_double_buffer
    ex05_multi_stream_sync
)
    add_executable(${ex} ${ex}.cu)
endforeach()
