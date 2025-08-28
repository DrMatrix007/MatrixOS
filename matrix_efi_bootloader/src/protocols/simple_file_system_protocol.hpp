#ifndef MATRIX_EFI_PROTOCOLS_SIMPLE_FILESYSTEM_PROTOCOL_H
#define MATRIX_EFI_PROTOCOLS_SIMPLE_FILESYSTEM_PROTOCOL_H

#include "efi.h"
#include "efiprot.h"
#include "protocol.hpp"
#include "protocols/simple_file_system_file.hpp"
#include "result.hpp"

namespace matrix_efi
{


class simple_filesystem_protocol
{
public:
    using raw = EFI_SIMPLE_FILE_SYSTEM_PROTOCOL;

    simple_filesystem_protocol(protocol_handle<raw> raw);
    simple_filesystem_protocol(simple_filesystem_protocol&&) = default;
    simple_filesystem_protocol& operator=(simple_filesystem_protocol&&) =
        default;

    static efi_guid guid();
    raw* get_raw();

    mst::result<simple_filesystem_file, efi_error> open_volume();

private:
    protocol_handle<raw> m_raw;
};

} // namespace matrix_efi

#endif // MATRIX_EFI_PROTOCOLS_SIMPLE_FILESYSTEM_PROTOCOL_H