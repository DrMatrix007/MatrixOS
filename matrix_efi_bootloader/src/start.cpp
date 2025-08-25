#include <efi.h>
#include <efilib.h>

#include "efierr.h"
#include "graphics/frame_buffer.hpp"
#include "match.hpp"
#include "protocols/graphics_protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle,
                                      EFI_SYSTEM_TABLE* SystemTable)
{

    ST = SystemTable;

    system_table table(SystemTable);

    mst::optional<simple_output_protocol&> out_opt = table.out();

    match_or(simple_out, out_opt, return -1);

    simple_out.clear_screen();
    simple_out.output_string((wchar_t*)L"hello world\n");

    auto gop_opt = table.get_protocol<graphics_protocol>();

    match_or(gop, gop_opt, return EFI_ABORTED);

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
    auto frame = gop.frame_buffer();
    
    table.exit_boot_services();
    
    
    for (int x = 0; x < frame.width(); x++)
    {
        
        for (int y = 0; y < frame.width(); y++)
        {
            frame.set_pixel(x, y, mbi::pixel{10, 10, 25, 10});
        }
    }

    while (true)
    {
    }

    return EFI_SUCCESS;
}
