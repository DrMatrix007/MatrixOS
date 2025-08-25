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
using raw_system_table = EFI_SYSTEM_TABLE;
using raw_efi_handle = EFI_HANDLE;
using efi_status = EFI_STATUS;

class system_table
{
public:
    system_table(raw_system_table* ptr, raw_efi_handle image_handle);
    template <efi_protocol protocol> mst::optional<protocol> get_protocol();
    template <efi_protocol protocol> void close_protocol(protocol prot);
    mst::optional<simple_output_protocol&> out();
    efi_status exit_boot_services();

private:
    raw_system_table* m_raw;
    raw_efi_handle m_image_handle;
    mst::optional<simple_output_protocol> m_out;
};

template <efi_protocol protocol>
inline mst::optional<protocol> system_table::get_protocol()
{
    efi_guid guid = protocol::guid();

    typename protocol::raw* inter;

    m_raw->BootServices->LocateProtocol(&guid, NULL, (void**)&inter);
    if (inter != nullptr)
    {
        return protocol(inter);
    }
    return mst::nullopt;
}

template <efi_protocol protocol>
inline void system_table::close_protocol(protocol prot)
{
    m_raw->BootServices->CloseProtocol(prot.get_raw(), protocol::guid(),
                                       nullptr, nullptr);
}

}; // namespace matrix_efi

#endif // MATRIX_EFI_SYSTEM_TABLE_H
