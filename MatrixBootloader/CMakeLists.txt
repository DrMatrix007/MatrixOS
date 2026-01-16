cmake_minimum_required(VERSION 3.28.0)
project(MatrixBootloader VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB_RECURSE SRC_MODULES "src/**.ixx")
file(GLOB_RECURSE SRC "src/**.c" "src/**.cpp" "src/**.h" "src/**.hpp" ${SRC_MODULES})

add_executable(MatrixBootloader ${SRC})
target_sources(MatrixBootloader PRIVATE FILE_SET CXX_MODULES FILES ${SRC_MODULES})

target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi")
target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi/x86_64")
target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi/protocol")

target_compile_options(MatrixBootloader PRIVATE
        -target x86_64-unknown-windows
        -ffreestanding
        -fshort-wchar
        -mno-red-zone
)

target_link_options(MatrixBootloader PRIVATE
        -target x86_64-unknown-windows
        -nostdlib
        -Wl,-entry:efi_main
        -Wl,-subsystem:efi_application
        -fuse-ld=lld-link
)
