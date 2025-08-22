#include <efi.h>
#include <efilib.h>

#include "match.hpp"
#include "protocols/graphics_protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "system_table.hpp"

using namespace matrix_efi;

static inline uint32_t rand32()
{
    static uint32_t state = 0x12345678; // seed
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    return state;
}

extern "C" EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle,
                                      EFI_SYSTEM_TABLE* SystemTable)
{
    EFI_STATUS status = EFI_SUCCESS;

    ST = SystemTable;

    system_table table(SystemTable);

    mst::optional<simple_output_protocol&> out_opt = table.out();

    match(simple_out, out_opt)
    {
        simple_out.clear_screen();
        simple_out.output_string((wchar_t*)L"hello world\n");
    }

    auto gop = table.get_protocol<graphics_protocol>();

    match(gop, gop)
    {
        auto res = gop.try_find_mode(1920, 1080);
        match(index, res)
        {

            gop.set_mode(index);

            while (true)
            {
                for (uint32 y = 0; y < 1080; ++y)
                {
                    for (uint32 x = 0; x < 1920; ++x)
                    {
                        uint32_t v = rand32();
                        uint8 r = (v >> 16) & 0xFF;
                        uint8 g = (v >> 8) & 0xFF;
                        uint8 b = v & 0xFF;

                        gop.draw_pixel(x, y, r, g, b);
                    }
                }
            }
        }
    }

    while (true)
    {
    }
    return status;
}
