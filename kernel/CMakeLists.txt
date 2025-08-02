cmake_minimum_required(VERSION 4.0)

include(ExternalProject)

project(matrix_kernel VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER /bin/clang++)
set(CMAKE_C_COMPILER /bin/clang)

file(GLOB_RECURSE SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE HEADER_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h")


add_executable(matrix_kernel ${SOURCE_FILES} ${HEADER_FILES})

target_link_libraries(matrix_kernel matrix_efi)

target_compile_features(matrix_kernel PUBLIC c_std_23)
target_compile_features(matrix_kernel PUBLIC cxx_std_26)

target_compile_options(matrix_kernel PRIVATE

    -target x86_64-unknown-windows 
    -ffreestanding 
    -fshort-wchar 
    -mno-red-zone 
    -I/usr/include
)

target_link_options(matrix_kernel PRIVATE
    -target x86_64-unknown-windows
    -nostdlib
    -Wl,-entry:efi_main
    -Wl,-subsystem:efi_application
    -fuse-ld=lld-link

)

set(IMAGE_NAME matrix_kernel.img)
set(ISO_PATH ${CMAKE_CURRENT_BINARY_DIR}/iso_build)
set(EXE_NAME matrix_kernel)


add_custom_command(
    DEPENDS matrix_kernel
    OUTPUT ${IMAGE_NAME}

    # Create empty 1.44MB FAT image
    COMMAND dd if=/dev/zero of=${IMAGE_NAME} bs=1k count=1440

    # Format the image as FAT12
    COMMAND mformat -i ${IMAGE_NAME} -f 1440 ::

    # Create EFI directories
    COMMAND mmd -i ${IMAGE_NAME} ::/EFI
    COMMAND mmd -i ${IMAGE_NAME} ::/EFI/BOOT

    # Copy your UEFI app
    COMMAND mcopy -i ${IMAGE_NAME} ${EXE_NAME} ::/EFI/BOOT/BOOTX64.EFI
    COMMAND mcopy -i ${IMAGE_NAME} ${CMAKE_CURRENT_SOURCE_DIR}/startup.nsh ::

    COMMENT "Creating UEFI FAT image with BOOTX64.EFI"
)

add_custom_target(build_image ALL
    DEPENDS ${IMAGE_NAME}
)

set(MATRIX_IMAGE_PATH ${CMAKE_CURRENT_BINARY_DIR}/${IMAGE_NAME})

include(CTest)
enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})


include(CPack)
