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
using raw_system_table = struct _EFI_SYSTEM_TABLE;
class system_table
{
public:
    system_table(raw_system_table* ptr);
    template <efi_protocol protocol> mst::optional<protocol> get_protocol();
    mst::optional<simple_output_protocol&> out();

private:
    raw_system_table* m_raw;
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

}; // namespace matrix_efi

#endif // MATRIX_EFI_SYSTEM_TABLE_H
