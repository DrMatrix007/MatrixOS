#include <efi/efi.h>
#include <efi/efilib.h>

extern EFI_SYSTEM_TABLE* ST = nullptr;

void printf(const wchar_t* data){
    ST->ConOut->OutputString(ST->ConOut, (wchar_t*)data);
}
