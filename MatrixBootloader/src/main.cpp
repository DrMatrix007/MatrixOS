#include <efi.h>
#include <efilib.h>

import MatrixEfiTable;
import standard_test;

extern "C" EFI_STATUS efi_main(EFI_HANDLE image_handle, EFI_SYSTEM_TABLE *system_table)
{
    ST = system_table;

    mefi::MatrixEfiTable table;

    if (2 != mtd::add(1,1))
    {
        return -1;
    }

    EFI_INPUT_KEY key;

    EFI_STATUS status = ST->ConIn->Reset(ST->ConIn, FALSE);
    if (EFI_ERROR(status))
        return status;


    while ((status = ST->ConIn->ReadKeyStroke(ST->ConIn, &key)) == EFI_NOT_READY) {}

    return EFI_SUCCESS;
}