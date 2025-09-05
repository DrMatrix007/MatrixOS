cmake_minimum_required(VERSION 3.10.0)
project(matrix_efi_bootloader VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER x86_64-w64-mingw32-g++)
set(CMAKE_C_COMPILER x86_64-w64-mingw32-gcc)


file(GLOB_RECURSE SRC src/*.cpp src/*.hpp gnu-efi-src/data.c)

add_executable(matrix_efi_bootloader ${SRC})

target_link_libraries(matrix_efi_bootloader PUBLIC standard_matrix)
target_link_libraries(matrix_efi_bootloader PUBLIC matrix_boot_info_uefi)


target_compile_features(matrix_efi_bootloader PUBLIC c_std_23)
target_compile_features(matrix_efi_bootloader PUBLIC cxx_std_26)

target_include_directories(matrix_efi_bootloader PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src/")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/x86_64")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/protocol")


target_compile_options(matrix_efi_bootloader PRIVATE
    -fno-exceptions
    # -fno-rtti only for cpp.
    -Werror 
    -m64
    -ffreestanding
)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti -fno-sized-deallocation")

target_link_options(matrix_efi_bootloader PRIVATE
    -lgcc
    -Werror
    -m64
    # -mabi=ms
    -nostdlib
    -shared
    -Wl,-dll
    -Wl,--subsystem,10
    -e efi_main
    -Wl,-o,matrix_efi_bootloader

)


include(CTest)
enable_testing()
