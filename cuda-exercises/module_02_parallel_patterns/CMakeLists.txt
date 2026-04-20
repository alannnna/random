foreach(ex
    ex01_reduction
    ex02_histogram
    ex03_prefix_scan
    ex04_stencil
    ex05_dot_product
)
    add_executable(${ex} ${ex}.cu)
endforeach()
