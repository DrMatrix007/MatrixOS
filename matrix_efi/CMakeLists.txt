cmake_minimum_required(VERSION 3.10.0)
project(matrix_efi VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER /bin/clang++)
set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SRC src/*.cpp src/*.hpp gnu-efi-src/data.c)

add_library(matrix_efi ${SRC})

target_include_directories(matrix_efi PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src/")
target_include_directories(matrix_efi PUBLIC "/usr/include/efi/")
target_include_directories(matrix_efi PUBLIC "/usr/include/efi/x86_64")
target_include_directories(matrix_efi PUBLIC "/usr/include/efi/protocol")

target_compile_features(matrix_efi PUBLIC c_std_23)
target_compile_features(matrix_efi PUBLIC cxx_std_26)

target_compile_options(matrix_efi PRIVATE
    -target x86_64-unknown-windows
    -ffreestanding
    -fshort-wchar
    -mno-red-zone
)

target_link_options(matrix_efi PRIVATE
    -lefi
    -lgnuefi
    -fuse-ld=lld-link
    -target x86_64-unknown-windows
    -nostdlib
    -Wl,-subsystem:efi_application

)


include(CTest)
enable_testing()



set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
