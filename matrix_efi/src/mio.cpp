#include <efi.h>
#include <efilib.h>

void print(const wchar_t* data){
    
    ST->ConOut->OutputString(ST->ConOut, (wchar_t*)data);
}
