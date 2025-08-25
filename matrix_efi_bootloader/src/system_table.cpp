#include "system_table.hpp"
#include "efidef.h"
#include "match.hpp"
#include "optional.hpp"

namespace matrix_efi
{
system_table::system_table(raw_system_table* ptr, raw_efi_handle image_handle)
    : m_raw(ptr), m_image_handle(image_handle)
{
    m_out = simple_output_protocol(ptr->ConOut);
    match(val, m_out)
    {
        val.output_string((wchar_t*)L"whatttt");
    }
}

mst::optional<matrix_efi::simple_output_protocol&> matrix_efi::system_table::
    out()
{
    return m_out.as_ref();
}

efi_status system_table::exit_boot_services()
{
    EFI_MEMORY_DESCRIPTOR* map = NULL;
    uint64 map_size, map_key, descriptor_size;
    uint32 descriptor_version;
    uint64 memory_size = 0;

    m_raw->BootServices->GetMemoryMap(&map_size, map, &map_key, &descriptor_size, &descriptor_version);
    return m_raw->BootServices->ExitBootServices(m_image_handle, map_key);
}

} // namespace matrix_efi