#if !defined(MATRIX_EFI_PROTOCOL_H)
#define MATRIX_EFI_PROTOCOL_H

#include <efi.h>

namespace matrix_efi
{
    using EfiGuid = EFI_GUID;

    template<typename T>
    concept EfiProtocol = requires() {
        { T::guid() };
    };

    
}

#endif // MATRIX_EFI_PROTOCOL_H
