#include "elf_loader.hpp"
#include "boot_services.hpp"
#include "int_types.hpp"
#include "match.hpp"
#include "system_table.hpp"

namespace matrix_efi
{
entry_func load_file(simple_filesystem_file& kernel)
{
    auto&& boot = g_system_table.boot_services();
    match_or(out, g_system_table.out(), return nullptr);

    elf_header header;
    uintn header_size = sizeof(header);

    kernel.set_position(0);

    kernel.read(&header, &header_size);

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
        void* ptr =
            reinterpret_cast<decltype(ptr)>(pheader.p_vaddr & 0xFFFFFFFFFF000);
        void* actual_ptr =
            reinterpret_cast<decltype(actual_ptr)>(pheader.p_vaddr);

        int64 pages_amount = (pheader.p_memsz + page_size - 1) / page_size;

        match(err,
              boot.allocate_pages(allocate_type::address,
                                  memory_type::loader_code, pages_amount, &ptr))
        {
            out.print(L"Error: %p %s ", ptr, err.as_string());
            return nullptr;
        }
        out.print(L"works ");

        // boot.set_mem(ptr, pages_amount * page_size,0);

        kernel.set_position(pheader.p_offset);
        uintn image_size = pheader.p_filesz;

        kernel.read(ptr, &image_size);
    }

    return (entry_func)header.e_entry;

    return nullptr;
}
} // namespace matrix_efi