cmake_minimum_required(VERSION 3.14)


set(CMAKE_C_FLAGS "-std=gnu99 -ffreestanding -O2 -Wall -Wextra")
set(CMAKE_CXX_FLAGS "-ffreestanding -Wall -Wextra -fno-exceptions -fno-rtti -nostdlib -nostartfiles") 
set(CMAKE_EXE_LINKER_FLAGS "-T ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld -o -ffreestanding -O2 -nostdlib -lgcc")
set(CMAKE_ASM_NASM_OBJECT_FORMAT elf32)

set_source_files_properties(bootstrap.s PROPERTIES LANGUAGE ASM_NASM)


project(matrix_os VERSION 0.1.0 LANGUAGES C CXX)

enable_language(ASM_NASM)


add_executable(matrix_os main.cpp bootstrap.s)


target_compile_features(matrix_os PUBLIC c_std_23)
target_compile_features(matrix_os PUBLIC cxx_std_26)

include(CTest)
enable_testing()