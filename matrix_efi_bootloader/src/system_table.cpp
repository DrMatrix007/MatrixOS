#include "optional.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"
#include "mio.hpp"

matrix_efi::system_table::system_table(raw_system_table *ptr) : m_raw(ptr)
{
    m_out = simple_output_protocol(ptr->ConOut);
    match(val, m_out)
    {
        val.output_string((wchar_t*)L"whatttt");
    } else
    {
        print((wchar_t*) L"this is bad");
    }
}

mst::optional<matrix_efi::simple_output_protocol&> matrix_efi::system_table::out()
{
    return mst::optional<matrix_efi::simple_output_protocol&>{};
}
