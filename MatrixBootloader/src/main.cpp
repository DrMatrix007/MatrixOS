#include <efi.h>
#include <efilib.h>

import MatrixEfiTable;

extern "C" EFI_STATUS efi_main(EFI_HANDLE image_handle, EFI_SYSTEM_TABLE *system_table)
{
    mefi::MatrixEfiTable table;

    return EFI_SUCCESS;
}