cmake_minimum_required(VERSION 3.28.0)
project(MatrixBootloader VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB_RECURSE SRC_MODULES "${CMAKE_CURRENT_SOURCE_DIR}/src/**.ixx")
file(GLOB_RECURSE SRC "${CMAKE_CURRENT_SOURCE_DIR}/src/**.c" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.hpp" ${SRC_MODULES})

add_executable(MatrixBootloader ${SRC})
target_sources(MatrixBootloader PRIVATE FILE_SET CXX_MODULES FILES ${SRC_MODULES})

target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi")
target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi/x86_64")
target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi/protocol")


target_link_libraries(MatrixBootloader PRIVATE EfiProject)
target_link_libraries(MatrixBootloader PRIVATE StandardMatrixEfi)