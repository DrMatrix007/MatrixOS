#if !defined(MATRIX_EFI_GRAPHICS_PROTOCOL_H)
#define MATRIX_EFI_GRAPHICS_PROTOCOL_H

#include "protocol.hpp"
#include "type_traits.hpp"

namespace matrix_efi
{

    class graphics_protocol
    {
    public:
        static efi_guid guid();
    };

}

#endif // MATRIX_EFI_GRAPHICS_PROTOCOL_H