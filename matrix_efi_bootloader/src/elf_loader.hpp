#ifndef MATRIX_EFI_BOOTLOADER_ELF_LOADER_H
#define MATRIX_EFI_BOOTLOADER_ELF_LOADER_H

#include "boot_info.hpp"
#include "protocols/simple_file_system_file.hpp"

namespace matrix_efi
{

struct elf_header
{
    union {
        struct
        {
            unsigned char ei_magic[4];
            unsigned char ei_class;
            unsigned char ei_data;
            unsigned char ei_version;
            unsigned char ei_osabi;
            unsigned char ei_abiversion;
        };
        unsigned char e_ident[16];
    };

    uint16 e_type;
    uint16 e_machine;
    uint32 e_version;
    uint64 e_entry;
    uint64 e_phoff;
    uint64 e_shoff;
    uint32 e_flags;
    uint16 e_ehsize;
    uint16 e_phentsize;
    uint16 e_phnum;
    uint16 e_shentsize;
    uint16 e_shnum;
    uint16 e_shstrndx;
};

struct elf_program_header
{
    uint32 p_type;
    uint32 p_flags;
    uint64 p_offset;
    uint64 p_vaddr;
    uint64 p_paddr;
    uint64 p_filesz;
    uint64 p_memsz;
    uint64 p_align;
};

struct elf_section_header
{
    uint32 sh_name;
    uint32 sh_type;
    uint64 sh_flags;
    uint64 sh_addr;
    uint64 sh_offset;
    uint64 sh_size;
    uint32 sh_link;
    uint32 sh_info;
    uint64 sh_addralign;
    uint64 sh_entsize;
};

static_assert(sizeof(elf_header) == 0x40, "this header should be of size 0x40!");
static_assert(sizeof(elf_program_header) == 0x38, "this header should be of size 0x38!");
static_assert(sizeof(elf_section_header) == 0x40, "this header should be of size 0x40!");

using entry_func = void (*)(mbi::boot_info info);

entry_func load_file(simple_filesystem_file& kernel);
} // namespace matrix_efi

#endif // MATRIX_EFI_BOOTLOADER_ELF_LOADER_H