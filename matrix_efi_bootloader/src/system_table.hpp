#include "efi_error.hpp"
#include "result.hpp"
#if !defined(MATRIX_EFI_SYSTEM_TABLE_H)
#define MATRIX_EFI_SYSTEM_TABLE_H

#include <efi.h>
#include <efiapi.h>
#include <efilib.h>

#include "optional.hpp"
#include "protocols/protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
namespace matrix_efi
{
using raw_efi_handle = EFI_HANDLE;
class system_table
{
public:
    using raw = EFI_SYSTEM_TABLE;
    system_table(raw* ptr, raw_efi_handle image_handle);
    template <efi_protocol protocol> mst::result<protocol, efi_error> get_protocol();
    template <efi_protocol protocol> void close_protocol(protocol prot);
    mst::optional<simple_output_protocol&> out();
    mst::optional<efi_error> exit_boot_services();

private:
    raw* m_raw;
    raw_efi_handle m_image_handle;
    mst::optional<simple_output_protocol> m_out;
};

template <efi_protocol protocol>
inline mst::result<protocol, efi_error> system_table::get_protocol()
{

    efi_guid guid = protocol::guid();

    typename protocol::raw* inter;

    efi_status err = m_raw->BootServices->LocateProtocol(&guid, nullptr, (void**)&inter);
    if (err == efi_success)
    {
        return protocol(protocol_handle<typename protocol::raw>(inter, m_raw, protocol::guid()));
    }
    return efi_error(err);
}

template <efi_protocol protocol>
inline void system_table::close_protocol(protocol prot)
{
    m_raw->BootServices->CloseProtocol(prot.get_raw(), protocol::guid(),
                                       nullptr, nullptr);
}

}; // namespace matrix_efi

#endif // MATRIX_EFI_SYSTEM_TABLE_H
