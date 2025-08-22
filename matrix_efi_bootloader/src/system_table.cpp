#include "optional.hpp"
#include "system_table.hpp"
#include "match.hpp"

matrix_efi::system_table::system_table(raw_system_table *ptr) : m_raw(ptr)
{
    m_out = simple_output_protocol(ptr->ConOut);
    match(val, m_out)
    {
        val.output_string((wchar_t*)L"whatttt");
    }
}

mst::optional<matrix_efi::simple_output_protocol&> matrix_efi::system_table::out()
{
    match(value, m_out)
    {
        return mst::optional<simple_output_protocol&>(value);
    }
    return mst::nullopt;
}
