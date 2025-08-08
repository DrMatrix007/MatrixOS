#include "efi.h"

#include "graphics_protocol.hpp"


matrix_efi::efi_guid matrix_efi::graphics_protocol::guid()
{
    return EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
}

