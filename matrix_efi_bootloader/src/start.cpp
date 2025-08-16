#include <efi.h>
#include <efilib.h>

#include "mio.hpp"
#include "protocols/protocol.hpp"
#include "system_table.hpp"

// using namespace matrix_efi;

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS status = EFI_SUCCESS;

    ST = SystemTable;

    // system_table table(SystemTable);

    SystemTable->ConOut->OutputString(SystemTable->ConOut, (wchar_t *)L"hello world!!!\n");

    return status;
}
