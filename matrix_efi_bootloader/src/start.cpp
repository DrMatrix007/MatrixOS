#include <efi.h>
#include <efilib.h>

#include "match.hpp"
#include "protocols/graphics_protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle,
                                      EFI_SYSTEM_TABLE* SystemTable)
{
    EFI_STATUS status = EFI_SUCCESS;

    ST = SystemTable;

    system_table table(SystemTable);

    mst::optional<simple_output_protocol&> out_opt = table.out();

    match_or(simple_out, out_opt, return -1);

    simple_out.clear_screen();
    simple_out.output_string((wchar_t*)L"hello world\n");

    auto gop_opt = table.get_protocol<graphics_protocol>();

    match_or(gop, gop_opt, return status);

    auto res = gop.try_find_mode(1920, 1080);
    match(index, res)
    {
        gop.set_mode(index);
    }

    match(simple_out, out_opt)
    {
        simple_out.clear_screen();
        simple_out.output_string((wchar_t*)L"hello world\n");
    }

    while (true)
    {
    }
    return status;
}
