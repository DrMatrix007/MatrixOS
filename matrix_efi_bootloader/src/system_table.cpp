#include "system_table.hpp"
#include "match.hpp"
#include "optional.hpp"

namespace matrix_efi
{
system_table::system_table(raw_system_table* ptr) : m_raw(ptr)
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

void system_table::exit_boot_services()
{
    m_raw->BootServices->ExitBootServices(nullptr, 0);
}

} // namespace matrix_efi