cmake_minimum_required(VERSION 3.31)

project(matrix_kernel_runner)

add_executable(matrix_kernel_runner main.cpp)

set(EFI_BOOTLOADER_PROJECT_PATH matrix_efi_bootloader)
set(BOOTLOADER_BIN ${EFI_BOOTLOADER_PROJECT_PATH}/matrix_efi_bootloader)

set(KERNEL_PROJECT_PATH matrix_kernel)
set(KERNEL_BIN ${KERNEL_PROJECT_PATH}/matrix_kernel)

set(STANDARD_MATRIX_PROJECT_PATH standard_matrix)

add_subdirectory(${STANDARD_MATRIX_PROJECT_PATH})
add_subdirectory(${EFI_BOOTLOADER_PROJECT_PATH})
add_subdirectory(${KERNEL_PROJECT_PATH})

target_compile_features(matrix_efi_bootloader PUBLIC c_std_23)
target_compile_features(matrix_efi_bootloader PUBLIC cxx_std_26)

target_compile_features(matrix_kernel PUBLIC c_std_23)
target_compile_features(matrix_kernel PUBLIC cxx_std_26)

target_link_libraries(matrix_kernel_runner standard_matrix)


# variables for commands
set(IMAGE_NAME matrix_kernel.img)
set(ISO_PATH ${CMAKE_CURRENT_BINARY_DIR}/iso_build)
set(EXE_NAME matrix_efi_bootloader/matrix_efi_bootloader)

# commands
add_custom_command(
    OUTPUT OVMF.fd
    COMMAND cp ${CMAKE_CURRENT_SOURCE_DIR}/OVMF.4m.fd ${CMAKE_CURRENT_BINARY_DIR}/OVMF.fd
)
add_custom_target(copy_ovmf ALL
    DEPENDS OVMF.fd
)


add_custom_command(
    OUTPUT ${IMAGE_NAME}

    COMMAND dd if=/dev/zero of=${IMAGE_NAME} bs=1k count=1440

    COMMAND mformat -i ${IMAGE_NAME} -f 1440 ::

    COMMAND mmd -i ${IMAGE_NAME} ::/EFI
    COMMAND mmd -i ${IMAGE_NAME} ::/EFI/BOOT

    COMMAND mcopy -i ${IMAGE_NAME} ${BOOTLOADER_BIN} ::/EFI/BOOT/BOOTX64.EFI
    COMMAND mcopy -i ${IMAGE_NAME} ${KERNEL_BIN} ::/EFI/BOOT/matrix_kernel
    COMMAND mcopy -i ${IMAGE_NAME} ${CMAKE_CURRENT_SOURCE_DIR}/${EFI_BOOTLOADER_PROJECT_PATH}/startup.nsh ::

    COMMENT "Creating UEFI FAT image"
)

add_custom_target(build_image ALL
    DEPENDS ${IMAGE_NAME} matrix_efi_bootloader
)

set(MATRIX_IMAGE_PATH ${CMAKE_CURRENT_BINARY_DIR}/${IMAGE_NAME})

add_dependencies(matrix_kernel_runner matrix_efi_bootloader matrix_kernel standard_matrix build_image copy_ovmf)
