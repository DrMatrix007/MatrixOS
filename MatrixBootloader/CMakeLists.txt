cmake_minimum_required(VERSION 3.10.0)
project(MatrixBootloader VERSION 0.1.0 LANGUAGES C CXX)

file(GLOB_RECURSE SRC "src/**.c" "src/**.cpp" "src/**.h" "src/**.hpp")

message(STATUS "Source files: ${SRC}")

add_executable(MatrixBootloaderBin ${SRC})

target_include_directories(MatrixBootloaderBin PRIVATE "/usr/include/efi")
target_include_directories(MatrixBootloaderBin PRIVATE "/usr/include/efi/x86_64")
target_include_directories(MatrixBootloaderBin PRIVATE "/usr/include/efi/protocol")

target_compile_options(MatrixBootloaderBin PRIVATE
        -ffreestanding
        -fshort-wchar
        -mno-red-zone
        -fno-stack-protector
        -fpic
        -fshort-wchar
)

target_link_options(MatrixBootloaderBin PRIVATE
        /lib/libgnuefi.a
        /lib/libefi.a
        /usr/lib/crt0-efi-x86_64.o
        -nostdlib
        -shared
        -Bsymbolic
        -fuse-ld=lld
        -T/usr/lib/elf_x86_64_efi.lds
        #    -Wl,-subsystem:efi_application
)

add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/MatrixBootloader
        COMMAND objcopy -j .text -j .sdata -j .data -j .rodata -j .dynamic -j .dynsym -j .rel -j .rela -j .reloc
        --subsystem=10 --target=efi-app-x86_64 ${CMAKE_CURRENT_BINARY_DIR}/MatrixBootloaderBin ${CMAKE_CURRENT_BINARY_DIR}/BOOTX64.efi
        COMMENT "Converting ELF to EFI"
        DEPENDS MatrixBootloaderBin
)

add_custom_target(MatrixBootloader DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/MatrixBootloader)
