#if !defined(MATRIX_EFI_SYSTEM_TABLE_H)
#define MATRIX_EFI_SYSTEM_TABLE_H

#include <efi.h>
#include <efilib.h>
#include <efiapi.h>

#include "optional.hpp"

// #include "protocols/protocol.hpp"
#include "protocols/simple_output_protocol.hpp"

namespace matrix_efi
{
    using raw_system_table = EFI_SYSTEM_TABLE;
    class system_table
    {
    public:
        system_table(raw_system_table *ptr);
        // template <efi_protocol protocol>
        // protocol get_protocol();
        mst::optional<simple_output_protocol>& out();
    private:
        raw_system_table *m_raw;
        mst::optional<simple_output_protocol> m_out;
    };

    // template <efi_protocol protocol>
    // inline protocol system_table::get_protocol()
    // {
    //     // m_raw->BootServices.proto
    // }

}

#endif // MATRIX_EFI_SYSTEM_TABLE_H
