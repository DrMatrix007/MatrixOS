#include <efi.h>
#include <efilib.h>

#include "match.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS status = EFI_SUCCESS;

    ST = SystemTable;

    system_table table(SystemTable);
    
    mst::optional<simple_output_protocol&> out_opt = table.out();

    match(simple_out, out_opt)
    {
        simple_out.clear_screen();
        simple_out.output_string((wchar_t*)L"lol bozo\n");
    }

    while(true){}
    return status;
}
