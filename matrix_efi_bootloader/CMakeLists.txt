cmake_minimum_required(VERSION 3.10.0)
project(matrix_efi_bootloader VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER /bin/clang++)
set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SRC src/*.cpp src/*.hpp gnu-efi-src/data.c)

add_executable(matrix_efi_bootloader ${SRC})

# target_link_directories(standard_matrix PUBLIC)
# target_include_directories(matrix_efi_bootloader PUBLIC standard_matrix)
target_link_libraries(matrix_efi_bootloader PRIVATE standard_matrix)


target_compile_features(matrix_efi_bootloader PUBLIC c_std_23)
target_compile_features(matrix_efi_bootloader PUBLIC cxx_std_26)

target_include_directories(matrix_efi_bootloader PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src/")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/x86_64")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/protocol")


target_compile_options(matrix_efi_bootloader PRIVATE
#    -Wall
    -target x86_64-unknown-windows
    -fshort-wchar
    -ffreestanding
    -mno-red-zone
    -msoft-float
)
    
target_link_options(matrix_efi_bootloader PRIVATE
    -Wall
    -Wl,-entry:efi_main 
    -nostdlib
    -target x86_64-unknown-windows
    -Wl,-subsystem:efi_application
    -fuse-ld=lld-link
)


include(CTest)
enable_testing()



set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
