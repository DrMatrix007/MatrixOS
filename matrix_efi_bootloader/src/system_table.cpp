#include "system_table.hpp"
#include "protocols/simple_output_protocol.hpp"
#include <optional.hpp>
using namespace matrix_efi;

system_table::system_table(raw_system_table *ptr) : m_raw(ptr)
{
    m_out = simple_output_protocol(ptr->ConOut);
    
}

mst::optional<matrix_efi::simple_output_protocol> &system_table::out()
{
    return m_out;
}
