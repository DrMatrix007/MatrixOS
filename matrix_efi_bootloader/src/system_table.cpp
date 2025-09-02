#include "system_table.hpp"
#include "efi_error.hpp"
#include "efidef.h"
#include "optional.hpp"

namespace matrix_efi
{

system_table::system_table(raw* ptr, raw_efi_handle image_handle)
    : m_raw(ptr), m_image_handle(image_handle),
      m_boot_services(ptr->BootServices)
{
    m_out = simple_output_protocol(ptr->ConOut);
}

mst::optional<matrix_efi::simple_output_protocol&> matrix_efi::system_table::
    out()
{
    return m_out.as_ref();
}

mst::optional<efi_error> system_table::exit_boot_services()
{
    EFI_MEMORY_DESCRIPTOR* map = NULL;
    uint64 map_size, map_key, descriptor_size;
    uint32 descriptor_version;
    uint64 memory_size = 0;

    m_raw->BootServices->GetMemoryMap(&map_size, map, &map_key,
                                      &descriptor_size, &descriptor_version);
    efi_status err =
        m_raw->BootServices->ExitBootServices(m_image_handle, map_key);

    return make_efi_result(err);
}

boot_services& system_table::boot_services()
{
    return m_boot_services;
}


system_table g_system_table(0, 0);

} // namespace matrix_efi