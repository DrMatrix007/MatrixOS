cmake_minimum_required(VERSION 3.31.0)
project(matrix_boot_info VERSION 0.1.0 LANGUAGES C CXX)


set(CMAKE_CXX_COMPILER /bin/clang++)
set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/**.cpp")
file(GLOB_RECURSE HEADER_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/**.hpp")


add_library(matrix_boot_info ${HEADER_FILES})
include_directories(matrix_boot_info ${HEADRES})
target_include_directories(matrix_boot_info PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)

target_link_libraries(matrix_boot_info PUBLIC standard_matrix)


target_compile_features(matrix_boot_info PUBLIC c_std_23)
target_compile_features(matrix_boot_info PUBLIC cxx_std_26)
