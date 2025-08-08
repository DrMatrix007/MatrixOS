#include <efi.h>
#include <efilib.h>

#include "mio.hpp"
#include "protocols/protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS status = EFI_SUCCESS;
    
    system_table table(SystemTable);


    return status;
}