#include <efi.h>
#include <efilib.h>
#include "multiboot_header.hpp"
#include "kernel.hpp"
#include "mio.hpp"


EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS Status = EFI_SUCCESS;
    EFI_INPUT_KEY Key;
    ST = SystemTable;

    
    printf(L"Welcome to MatrixOS");
    while ((Status = ST->ConIn->ReadKeyStroke(ST->ConIn, &Key)) == EFI_NOT_READY) ;


    return EFI_SUCCESS;
}