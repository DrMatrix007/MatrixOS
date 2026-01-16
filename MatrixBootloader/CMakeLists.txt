cmake_minimum_required(VERSION 3.10.0)
project(MatrixBootloader VERSION 0.1.0 LANGUAGES C CXX)

file(GLOB_RECURSE SRC "src/**.c" "src/**.cpp" "src/**.h" "src/**.hpp")

message(STATUS "Source files: ${SRC}")

add_executable(MatrixBootloader ${SRC})

target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi")
target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi/x86_64")
target_include_directories(MatrixBootloader PRIVATE "/usr/include/efi/protocol")

target_compile_options(MatrixBootloader PRIVATE
    -target x86_64-unknown-windows
    -ffreestanding
    -fshort-wchar
    -mno-red-zone
    -Ipath/to/gnu-efi/inc -Ipath/to/gnu-efi/inc/x86_64 -Ipath/to/gnu-efi/inc/protocol
)

target_link_options(MatrixBootloader PRIVATE
    -target x86_64-unknown-windows
    -nostdlib
    -Wl,-entry:efi_main
    -Wl,-subsystem:efi_application
    -fuse-ld=lld-link
)
