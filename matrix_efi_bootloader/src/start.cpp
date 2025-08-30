#include <efi.h>
#include <efilib.h>

#include "efi_error.hpp"
#include "efierr.h"
#include "efiprot.h"
#include "match.hpp"
#include "protocols/graphics_protocol.hpp"
#include "protocols/simple_file_system_protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle,
                                      EFI_SYSTEM_TABLE* SystemTable)
{
    wchar_t kernel_path[] = L"\\EFI\\BOOT\\matrix_kernel";

    ST = SystemTable;

    system_table table(SystemTable, ImageHandle);

    mst::optional<simple_output_protocol&> out_opt = table.out();

    match_or(simple_out, out_opt, return -1);

    simple_out.clear_screen();

    auto gop_opt = table.locate_protocol<graphics_protocol>();
    simple_out.output_string((wchar_t*)L"hello world\n");

    match_or(gop, gop_opt, return EFI_ABORTED);

    auto index = gop.try_find_mode(1920, 1080);

    match(index, index)
    {
        gop.set_mode(index);
    }

    auto fs_opt = table.locate_protocol<simple_filesystem_protocol>();
    match(efi_error, err, fs_opt)
    {
        simple_out.print(L"%s\n", err.as_string());
        return EFI_ABORTED;
    }
    match_or(fs, fs_opt, return EFI_ABORTED);

    match_or(vol, fs.open_volume(), return EFI_ABORTED);


    match_or(kernel_file, vol.open(kernel_path, EFI_FILE_MODE_READ, 0), return EFI_ABORTED);

    simple_out.print(L"got root!");
    kernel_file.set_position(-1);
    match_or(pos, kernel_file.get_position(), return EFI_ABORTED);
    simple_out.print(L"%d", pos);


    return efi_success;
}
