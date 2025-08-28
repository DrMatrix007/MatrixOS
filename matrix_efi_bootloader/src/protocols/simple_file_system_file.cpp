#include "simple_file_system_file.hpp"
#include "int_types.hpp"

namespace matrix_efi
{

simple_filesystem_file::simple_filesystem_file(unique_handle<raw> raw)
    : m_raw(mst::move(raw))
{
}

mst::result<simple_filesystem_file, efi_error> simple_filesystem_file::open(
    wchar_t* filename, uint64 open_mode, uint64 attributes)
{
    simple_filesystem_file::raw* new_raw = nullptr;
    EFI_STATUS status =
        m_raw->Open(m_raw.get(), &new_raw, filename, open_mode, attributes);
    if (status != efi_success)
    {
        return efi_error(status);
    }
    return simple_filesystem_file(unique_handle<raw>(new_raw));
}

efi_result simple_filesystem_file::close()
{
    return make_efi_result(m_raw->Close(m_raw.get()));
}

efi_result simple_filesystem_file::remove()
{
    return make_efi_result(m_raw->Delete(m_raw.get()));
}

efi_result simple_filesystem_file::read(void* buffer, uintn* buffer_size)
{
    return make_efi_result(m_raw->Read(m_raw.get(), buffer_size, buffer));
}

efi_result simple_filesystem_file::write(void* buffer, uintn* buffer_size)
{
    return make_efi_result(m_raw->Write(m_raw.get(), buffer_size, buffer));
}

mst::result<uint64, efi_error> simple_filesystem_file::get_position()
{
    UINTN pos = 0;
    EFI_STATUS status = m_raw->GetPosition(m_raw.get(), &pos);
    if (status != efi_success)
    {
        return efi_error(status);
    }
    return (uint64)pos;
}

efi_result simple_filesystem_file::set_position(uint64 position)
{
    return efi_result(m_raw->SetPosition(m_raw.get(), position));
}

efi_result simple_filesystem_file::flush()
{
    return efi_result(m_raw->Flush(m_raw.get()));
}

simple_filesystem_file::~simple_filesystem_file()
{
    if (m_raw)
    {
        m_raw->Close(m_raw.get());
    }
}
} // namespace matrix_efi