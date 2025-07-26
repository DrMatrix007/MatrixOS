cmake_minimum_required(VERSION 3.10.0)
project(LibMatrix VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_COMPILER clang)


file(GLOB_RECURSE SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE HEADER_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h")


add_library(LibMatrix ${SOURCE_FILES} ${HEADER_FILES})

target_compile_features(LibMatrix PUBLIC c_std_23)
target_compile_features(LibMatrix PUBLIC cxx_std_26)


target_compile_options(LibMatrix PRIVATE
    -ffreestanding
    -Wall
    -Wextra
)

target_link_options(LibMatrix PRIVATE
    -nostdlib
    -ffreestanding
)


include(CTest)
enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
