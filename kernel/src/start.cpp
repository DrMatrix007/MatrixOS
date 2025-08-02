#include "multiboot_header.h"
#include "kernel.h"
#include "efi/efi.h"
#include "efi/efilib.h"
#include "../../matrix_efi/src/mio.hpp"


EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    ST = SystemTable;
    printf(L"test");

    return EFI_SUCCESS;
}