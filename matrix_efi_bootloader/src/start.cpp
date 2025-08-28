#include <efi.h>
#include <efilib.h>

#include "efi_error.hpp"
#include "efierr.h"
#include "efiprot.h"
#include "match.hpp"
#include "protocols/graphics_protocol.hpp"
#include "protocols/simple_file_system_file.hpp"
#include "protocols/simple_file_system_protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle,
                                      EFI_SYSTEM_TABLE* SystemTable)
{

    ST = SystemTable;

    system_table table(SystemTable, ImageHandle);

    mst::optional<simple_output_protocol&> out_opt = table.out();

    match_or(simple_out, out_opt, return -1);

    simple_out.clear_screen();
    simple_out.output_string((wchar_t*)L"hello world\n");

    auto gop_opt = table.get_protocol<graphics_protocol>();

    match_or(gop, gop_opt, return EFI_ABORTED);

    auto index = gop.try_find_mode(1920, 1080);

    match(index, index)
    {
        gop.set_mode(index);
    }

    auto fs_opt = table.get_protocol<simple_filesystem_protocol>();
    match(efi_error, err, fs_opt)
    {
        simple_out.print(L"%d\n", err);
        while (true)
        {
        }
    }
    match_or(fs, fs_opt, return EFI_ABORTED);
    simple_out.print(L"got fs! ");

    match_or(vol, fs.open_volume(), return EFI_ABORTED);
    simple_out.print(L"got vol! ");


    // match_or(
    //     kernel_file,
    //     vol.open((wchar_t*)L"fs0:/EFI/BOOT/BOOTX64.EFI", EFI_FILE_MODE_READ, 0),
    //     return EFI_ABORTED);
    simple_out.print(L"got kernel!\n");

    auto frame = gop.frame_buffer();

    auto res = table.exit_boot_services();

    match(res, res)
    {
        simple_out.print(L"Cant exit BootServices, Error: %d", res);
        return res.raw();
    }

    for (int x = 0; x < frame.width(); x++)
    {

        for (int y = 0; y < frame.width(); y++)
        {
            frame.set_pixel(x, y, mbi::pixel{10, 10, 25, 10});
        }
    }
    return efi_success;
}
