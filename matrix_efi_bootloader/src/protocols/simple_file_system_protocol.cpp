#include "simple_file_system_protocol.hpp"

#include "efi_error.hpp"
#include "efilib.h"
#include "result.hpp"
#include "unique_handle.hpp"

namespace matrix_efi
{

simple_filesystem_protocol::simple_filesystem_protocol(protocol_handle<raw> raw)
    : m_raw(mst::move(raw))
{
}

efi_guid simple_filesystem_protocol::guid()
{
    return gEfiSimpleFileSystemProtocolGuid;
}

mst::result<simple_filesystem_file,efi_error> simple_filesystem_protocol::open_volume()
{
    simple_filesystem_file::raw* ptr = nullptr;
    efi_status res = m_raw->OpenVolume(m_raw.get(), &ptr);
    if(res != efi_success)
    {
        return efi_error(res);
    }
    return simple_filesystem_file{ptr};
}

} // namespace matrix_efi