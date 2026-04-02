foreach(ex
    ex01_data_race
    ex02_deadlock
    ex03_condition_var
    ex04_atomics
    ex05_rwlock
)
    add_executable(${ex} ${ex}.cpp)
    target_link_libraries(${ex} PRIVATE Threads::Threads)
endforeach()
