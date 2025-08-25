cmake_minimum_required(VERSION 3.10.0)
project(matrix_efi_bootloader VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_COMPILER clang)


file(GLOB_RECURSE SRC src/*.cpp src/*.hpp gnu-efi-src/data.c)

add_executable(matrix_efi_bootloader ${SRC})

target_link_libraries(matrix_efi_bootloader PUBLIC standard_matrix)
target_link_libraries(matrix_efi_bootloader PUBLIC matrix_boot_info)


target_compile_features(matrix_efi_bootloader PUBLIC c_std_23)
target_compile_features(matrix_efi_bootloader PUBLIC cxx_std_26)

target_include_directories(matrix_efi_bootloader PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src/")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/x86_64")
target_include_directories(matrix_efi_bootloader PUBLIC "/usr/include/efi/protocol")


target_compile_options(matrix_efi_bootloader PRIVATE
    -Wall
    -target x86_64-unknown-windows
    -fshort-wchar
    -ffreestanding
    -mno-red-zone
    -fno-exceptions 
    -fno-rtti
    # -msoft-float
    # -fno-exceptions
    # -ffreestanding
    # -fshort-wchar
    # -mno-red-zone
)

target_link_options(matrix_efi_bootloader PRIVATE
    -Wall
    -Wl,-entry:efi_main
    -nostdlib
    -target x86_64-unknown-windows
    -Wl,-subsystem:efi_application
    -fuse-ld=lld
    # -nostdlib
    # -Wl,-dll
    # -shared
    # -Wl,--subsystem,10
    # -e efi_main
    # -T${CMAKE_CURRENT_SOURCE_DIR}/linker.ld
)
include(CTest)
enable_testing()


add_custom_target(matrix_efi_bootloader_efi ALL
    # Convert binary to EFI
    COMMAND objcopy
    --target=pei-x86-64
    ${CMAKE_CURRENT_BINARY_DIR}/matrix_efi_bootloader
    ${CMAKE_CURRENT_BINARY_DIR}/matrix_efi_bootloader.efi
    COMMENT "Converting binary to EFI"
    DEPENDS matrix_efi_bootloader # ensures the original binary is built first
)
add_custom_target(matrix_efi_bootloader_efi_subsystem ALL
    COMMAND ${CMAKE_COMMAND} -E echo "Patching EFI subsystem using Python..."
    # COMMAND python3 ${CMAKE_CURRENT_SOURCE_DIR}/subsystem_patch.py ${CMAKE_CURRENT_BINARY_DIR}/matrix_efi_bootloader.efi
    COMMENT "Patching EFI subsystem"
    DEPENDS matrix_efi_bootloader_efi
)







set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
