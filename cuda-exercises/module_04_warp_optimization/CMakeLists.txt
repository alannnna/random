foreach(ex
    ex01_transpose
    ex02_tiled_matmul
    ex03_warp_reduce
    ex04_grid_stride
    ex05_warp_sync
)
    add_executable(${ex} ${ex}.cu)
endforeach()
