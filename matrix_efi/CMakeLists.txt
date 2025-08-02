cmake_minimum_required(VERSION 3.10.0)
project(matrix_efi VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER /bin/clang++)
set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SRC src/*.cpp src/*.hpp )

add_library(matrix_efi ${SRC})

target_compile_features(matrix_efi PUBLIC c_std_23)
target_compile_features(matrix_efi PUBLIC cxx_std_26)

target_compile_options(matrix_efi PRIVATE
    -target x86_64-unknown-windows
    -ffreestanding
    -fshort-wchar
    -mno-red-zone
    -I/usr/include
)

target_link_options(matrix_efi PRIVATE
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
