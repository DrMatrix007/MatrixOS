#include <efi.h>
#include <efilib.h>
#include "multiboot_header.hpp"
#include "kernel.hpp"
#include "mio.hpp"
#include "mefi_start.hpp"

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    return setup(ImageHandle, SystemTable);
}