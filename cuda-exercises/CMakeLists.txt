cmake_minimum_required(VERSION 3.18)
project(cuda_exercises CUDA CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CUDA_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CUDA_STANDARD_REQUIRED ON)

# Override with: cmake -DCMAKE_CUDA_ARCHITECTURES=86 ..
if(NOT DEFINED CMAKE_CUDA_ARCHITECTURES)
    set(CMAKE_CUDA_ARCHITECTURES 70)
endif()

add_subdirectory(module_01_memory)
add_subdirectory(module_02_parallel_patterns)
add_subdirectory(module_03_streams)
add_subdirectory(module_04_warp_optimization)
