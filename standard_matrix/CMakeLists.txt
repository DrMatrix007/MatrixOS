cmake_minimum_required(VERSION 3.10.0)
project(standard_matrix VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER /bin/clang++)
set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SRC src/*.cpp gnu-efi-src/data.c)
file(GLOB_RECURSE HEADERS src/*.hpp gnu-efi-src/data.c)

add_library(standard_matrix ${SRC} ${HEADERS})
include_directories(standard_matrix ${HEADERS})
target_include_directories(standard_matrix PUBLIC src/)



target_compile_features(standard_matrix PUBLIC c_std_23)
target_compile_features(standard_matrix PUBLIC cxx_std_26)

target_compile_options(standard_matrix PRIVATE -Wall)
target_link_options(standard_matrix PRIVATE -Wall)

