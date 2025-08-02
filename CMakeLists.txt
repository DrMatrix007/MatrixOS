cmake_minimum_required(VERSION 3.31)

project(matrix_kernel_runner)

set(CMAKE_CXX_STANDARD 26)

add_executable(matrix_kernel_runner main.cpp)

add_subdirectory(./kernel)
