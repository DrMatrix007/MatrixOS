cmake_minimum_required(VERSION 3.28.0)
project(MatrixOS VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(EfiProject INTERFACE)
target_compile_options(EfiProject INTERFACE
        -target x86_64-unknown-windows
        -ffreestanding
        -fshort-wchar
        -mno-red-zone
)
target_link_options(EfiProject INTERFACE
        -target x86_64-unknown-windows
        -nostdlib
        -Wl,-entry:efi_main
        -Wl,-subsystem:efi_application
        -fuse-ld=lld-link
)

add_library(ElfProject INTERFACE)
target_compile_options(ElfProject INTERFACE)
target_link_options(ElfProject INTERFACE)




add_subdirectory(StandardMatrix)
add_subdirectory(MatrixBootloader)
add_subdirectory(MatrixKernel)

get_target_property(BOOTLOADER_BUILD_DIR MatrixBootloader BINARY_DIR)
set(BOOTLOADER_BIN ${BOOTLOADER_BUILD_DIR}/MatrixBootloader)

set(ISO_STAGING_DIR ${CMAKE_CURRENT_BINARY_DIR}/iso_root)
set(FAT_IMAGE ${CMAKE_CURRENT_BINARY_DIR}/image.img)
set(OVMF_FILE ${CMAKE_CURRENT_BINARY_DIR}/ovmf.fd)

add_custom_command(OUTPUT ${FAT_IMAGE}
    DEPENDS MatrixBootloader

    COMMAND dd if=/dev/zero of=${FAT_IMAGE} bs=1k count=1440
    COMMAND mformat -i ${FAT_IMAGE} -f 1440 ::
    COMMAND mmd -i ${FAT_IMAGE} ::/EFI
    COMMAND mmd -i ${FAT_IMAGE} ::/EFI/BOOT
    COMMAND mcopy -i ${FAT_IMAGE} ${BOOTLOADER_BIN} ::/EFI/BOOT/BOOTX64.EFI
    COMMENT "Generating bootable ISO image"
    USES_TERMINAL
)
add_custom_command(OUTPUT ${OVMF_FILE}
    COMMAND cp ${CMAKE_CURRENT_SOURCE_DIR}/ovmf/OVMF.4m.fd ${OVMF_FILE}
    COMMENT "Copying the OVMF file"
)
add_custom_target(Image ALL DEPENDS ${FAT_IMAGE} ${OVMF_FILE})

add_executable(MatrixOSRunner runner.cpp)
target_compile_definitions(
    MatrixOSRunner
    PRIVATE IMG_PATH="${FAT_IMAGE}"
)

add_dependencies(MatrixOSRunner Image)
add_custom_target(MatrixOS DEPENDS MatrixOSRunner)
