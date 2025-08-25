cmake_minimum_required(VERSION 4.0)


project(matrix_kernel VERSION 0.1.0 LANGUAGES C CXX)

# set(CMAKE_CXX_COMPILER /bin/clang++)
# set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE HEADER_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.hpp")


add_executable(matrix_kernel ${SOURCE_FILES})

target_compile_features(matrix_kernel PUBLIC c_std_23)
target_compile_features(matrix_kernel PUBLIC cxx_std_26)

target_compile_options(matrix_kernel PUBLIC
    -Wall
    -Werror
    -ffreestanding
)

target_link_options(matrix_kernel PUBLIC
    -nostdlib
)
