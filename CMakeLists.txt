cmake_minimum_required(VERSION 3.31)

project(matrix_kernel_runner)

set(CMAKE_CXX_STANDARD 26)

add_executable(matrix_kernel_runner main.cpp)

add_subdirectory(./matrix_efi)
add_subdirectory(./kernel)

add_custom_command(
    DEPENDS matrix_kernel_runner
    OUTPUT OVMF.fd
    COMMAND cp ${CMAKE_CURRENT_SOURCE_DIR}/OVMF.4m.fd ${CMAKE_CURRENT_BINARY_DIR}/OVMF.fd
)

add_custom_target(copy_ovmf ALL
    DEPENDS OVMF.fd
)