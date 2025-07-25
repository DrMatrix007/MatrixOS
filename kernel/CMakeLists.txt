cmake_minimum_required(VERSION 3.10.0)


set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_COMPILER clang)

project(matrix_kernel_image VERSION 0.1.0 LANGUAGES C CXX)

set(GNU_EFI_LIB "/lib/gnuefi/apps")

add_executable(matrix_kernel main.cpp)

target_compile_features(matrix_kernel PUBLIC c_std_23)
target_compile_features(matrix_kernel PUBLIC cxx_std_26)

target_compile_options(matrix_kernel PRIVATE
    -ffreestanding
    -fPIC
    -nostdlib
    -fno-stack-protector 
    -fno-stack-check 
    -fshort-wchar 
    -mno-red-zone 
    -target x86_64-efi
)

target_link_options(matrix_kernel PRIVATE 
    -fuse-ld=lld
    -nostdlib 
    -ffreestanding
    -lefi
    -lgnuefi
    -Wl,--subsystem=efi_application
)

set(IMAGE_NAME BOOTX64.efi)
set(EFI_BINARY matrix_kernel)

add_custom_command(
    OUTPUT ${IMAGE_NAME}
    COMMAND dd if=/dev/zero of=${IMAGE_NAME} bs=1k count=1440
    COMMAND mformat -i ${IMAGE_NAME} -f 1440 ::
    COMMAND mmd -i ${IMAGE_NAME} ::/EFI
    COMMAND mmd -i ${IMAGE_NAME} ::/EFI/BOOT
    COMMAND mcopy -i ${IMAGE_NAME} ${EFI_BINARY} ::/EFI/BOOT/${EFI_BINARY}.efi
    COMMENT "Creating FAT image with EFI bootloader"
)

add_custom_target(create_image ALL
    DEPENDS ${IMAGE_NAME}
)

include(CTest)
enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})


include(CPack)
