cmake_minimum_required(VERSION 3.14)
project(cpp_exercises CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Threads REQUIRED)

add_subdirectory(module_01_data_structures)
add_subdirectory(module_02_threads)
add_subdirectory(module_03_concurrency_patterns)
add_subdirectory(module_04_io_caching)
