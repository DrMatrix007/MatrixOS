#if !defined(MATRIX_EFI_PROTOCOL_H)
#define MATRIX_EFI_PROTOCOL_H

#include <efi.h>
#include <type_traits.hpp>

namespace matrix_efi
{
    using efi_guid = EFI_GUID;

    template<typename t>
    concept efi_protocol = requires() {
        { t::guid() } -> mst::same_as<efi_guid>;
    };
    
}

#endif // MATRIX_EFI_PROTOCOL_H
