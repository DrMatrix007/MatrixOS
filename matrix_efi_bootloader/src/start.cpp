#include <efi.h>
#include <efilib.h>

#include "boot_info.hpp"
#include "efi_error.hpp"
#include "efiapi.h"
#include "efierr.h"
#include "efiprot.h"
#include "elf_loader.hpp"
#include "match.hpp"
#include "protocols/graphics_protocol.hpp"
#include "protocols/simple_file_system_protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

uint64_t read_cr3() {
    uint64_t value;
    asm volatile("mov %%cr3, %0" : "=r"(value));
    return value;
}
uint64_t read_cr0() {
    uint64_t value;
    asm volatile("mov %%cr0, %0" : "=r"(value));
    return value;
}

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle,
                                      EFI_SYSTEM_TABLE* SystemTable)
{
    wchar_t kernel_path[] = L"\\EFI\\BOOT\\matrix_kernel";

    ST = SystemTable;

    g_system_table = system_table(SystemTable, ImageHandle);
    mst::optional<simple_output_protocol&> out_opt = g_system_table.out();

    match_or(out, out_opt, return -1);

    out.clear_screen();

    auto gop_opt = g_system_table.locate_protocol<graphics_protocol>();
    out.output_string((wchar_t*)L"hello world\n");

    match_or(gop, gop_opt, return EFI_ABORTED);

    auto index = gop.try_find_mode(1920, 1080);

    match(index, index)
    {
        gop.set_mode(index);
    }

    auto fs_opt = g_system_table.locate_protocol<simple_filesystem_protocol>();
    match(efi_error, err, fs_opt)
    {
        out.print(L"%s\n", err.as_string());
        return EFI_ABORTED;
    }
    match_or(fs, fs_opt, return EFI_ABORTED);

    match_or(vol, fs.open_volume(), return EFI_ABORTED);

    match_or(kernel_file, vol.open(kernel_path, EFI_FILE_MODE_READ, 0),
             return EFI_ABORTED);

    entry_func entry = load_file(kernel_file);

    mbi::boot_info info(gop.frame_buffer());
 
    g_system_table.exit_boot_services();
    // this is sysv ABI type shit
    // this calls to the entry fn
    asm volatile("mov %0, %%rdi\n"
                 "call *%1\n"
                 :
                 : "r"(&info), "r"(entry)
                 : "rdi");

    return efi_success;
}

// for static stuff, not relevant anyway. the kernel doesnt stop, and if it
// stops, the pc is off.
extern "C" void atexit()
{
}