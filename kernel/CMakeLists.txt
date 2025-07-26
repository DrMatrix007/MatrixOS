cmake_minimum_required(VERSION 3.10.0)


set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_COMPILER clang)

project(matrix_kernel VERSION 0.1.0 LANGUAGES C CXX)


file(GLOB_RECURSE SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB_RECURSE HEADER_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h")

add_executable(matrix_kernel ${SOURCE_FILES} ${HEADER_FILES})

target_compile_features(matrix_kernel PUBLIC c_std_23)
target_compile_features(matrix_kernel PUBLIC cxx_std_26)

target_compile_options(matrix_kernel PRIVATE
    -ffreestanding
    -Wall
    -Wextra
    -m32
)

target_link_options(matrix_kernel PRIVATE 
    -m32
    -nostdlib 
    -ffreestanding
    -T ${CMAKE_SOURCE_DIR}/linker.ld
)

set(IMAGE_NAME matrix_kernel.img)
set(ISO_PATH ${CMAKE_CURRENT_BINARY_DIR}/iso_build)
set(EXE_NAME matrix_kernel)


add_custom_command(
    OUTPUT ${IMAGE_NAME}

    COMMAND  mkdir -p ${ISO_PATH}/boot/grub/
    COMMAND  cp ${EXE_NAME} ${ISO_PATH}/boot/${EXE_NAME}
    COMMAND  cp ${CMAKE_SOURCE_DIR}/grub.cfg ${ISO_PATH}/boot/grub/grub.cfg
    COMMAND  grub-mkrescue -o ${IMAGE_NAME} ${ISO_PATH}

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
