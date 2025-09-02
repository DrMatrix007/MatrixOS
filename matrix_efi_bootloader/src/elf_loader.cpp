#include "elf_loader.hpp"
#include "boot_services.hpp"
#include "efidef.h"
#include "int_types.hpp"
#include "match.hpp"
#include "system_table.hpp"

namespace matrix_efi
{
entry_func load_file(simple_filesystem_file& kernel)
{
    match_or(out, g_system_table.out(), return nullptr);

    elf_header header;
    uintn header_size = sizeof(header);

    kernel.set_position(0);

    kernel.read(&header, &header_size);

    out.print(L"_%d_ ", header.e_shnum);

    for (int64 i = 0; i < header.e_phnum; i++)
    {
        kernel.set_position(header.e_phoff + i * sizeof(elf_program_header));
        elf_program_header pheader;
        uintn pheader_size = sizeof(pheader);
        kernel.read(&pheader, &pheader_size);
        if (pheader.p_type != elf_segment_type::load)
        {
            continue;
        }
        void* ptr = reinterpret_cast<decltype(ptr)>(pheader.p_vaddr);

        g_system_table.boot_services().allocate_pages(
            allocate_type::address, memory_type::loader_code,
            (pheader.p_memsz + page_size - 1) / page_size, &ptr);

        kernel.set_position(pheader.p_offset);
        uintn image_size = pheader.p_filesz;
        kernel.read(ptr, &image_size);
    }
    
    out.print(L"(%d)",     ((int32(*)())header.e_entry)());


    return nullptr;
}
} // namespace matrix_efi