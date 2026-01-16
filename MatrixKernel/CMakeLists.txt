cmake_minimum_required(VERSION 3.28.0)
project(MatrixKernel VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB_RECURSE SRC_MODULES "${CMAKE_CURRENT_SOURCE_DIR}/src/**.ixx")
file(GLOB_RECURSE SRC "${CMAKE_CURRENT_SOURCE_DIR}/src/**.c" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.hpp" ${SRC_MODULES})

add_executable(MatrixKernel ${SRC})
target_sources(MatrixKernel PRIVATE FILE_SET CXX_MODULES FILES ${SRC_MODULES})

target_link_libraries(MatrixKernel PRIVATE StandardMatrixElf)
