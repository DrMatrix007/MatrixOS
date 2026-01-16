cmake_minimum_required(VERSION 3.28.0)
project(StandardMatrix VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE SRC_MODULES "src/**.ixx")
file(GLOB_RECURSE SRC "src/**.c" "src/**.cpp" "src/**.h" "src/**.hpp" ${SRC_MODULES})

add_executable(StandardMatrix ${SRC})
target_sources(StandardMatrix PRIVATE FILE_SET CXX_MODULES FILES ${SRC_MODULES})
