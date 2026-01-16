cmake_minimum_required(VERSION 3.28.0)
project(StandardMatrix VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE SRC_MODULES "${CMAKE_CURRENT_SOURCE_DIR}/src/**.ixx")
file(GLOB_RECURSE SRC "${CMAKE_CURRENT_SOURCE_DIR}/src/**.c" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.h" "${CMAKE_CURRENT_SOURCE_DIR}/src/**.hpp")

add_library(StandardMatrixEfi ${SRC_MODULES})
target_sources(StandardMatrixEfi PUBLIC FILE_SET CXX_MODULES FILES ${SRC_MODULES})
target_link_libraries(StandardMatrixEfi PUBLIC EfiProject)

add_library(StandardMatrixElf ${SRC_MODULES})
target_sources(StandardMatrixElf PUBLIC FILE_SET CXX_MODULES FILES ${SRC_MODULES})
target_link_libraries(StandardMatrixEfi PUBLIC ElfProject)


