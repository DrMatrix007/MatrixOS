#include "efi.h"
#include "efiprot.h"

#include "graphics_protocol.hpp"
#include "int_types.hpp"
#include "match.hpp"
#include "optional.hpp"
#include "protocols/protocol.hpp"

using namespace matrix_efi;

efi_guid graphics_protocol::guid()
{
    return EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
}

graphics_protocol::graphics_protocol(protocol_handle<raw> raw) : m_raw(mst::move(raw))
{
}

mst::optional<int> graphics_protocol::try_find_mode(
    uint32_t width, uint32_t height,
    mst::optional<simple_output_protocol&> verbose)
{
    UINTN sizeofinfo = 0;
    EFI_GRAPHICS_OUTPUT_MODE_INFORMATION* info = nullptr;
    for (uint32_t i = 0; i < m_raw->Mode->MaxMode; ++i)
    {
        m_raw->QueryMode(m_raw.get(), i, &sizeofinfo, &info);

        // Print(L"size: %d %d\n", info->HorizontalResolution,
        // info->VerticalResolution);
        match(output, verbose)
        {
            // output.output_string((wchar_t*)L"whhjghzkjhzxkjhkjhfsjkfhsdkjfhsdkjfhsjjat?\n");
            output.print(L"size: %d %d\n", info->HorizontalResolution,
                         info->VerticalResolution);
        }

        if (info->HorizontalResolution == width &&
            info->VerticalResolution == height)
        {
            return i;
            break;
        }
    }
    return mst::nullopt;
}

void graphics_protocol::get_mode(uint32_t& width, uint32_t& height) const
{
    width = m_raw->Mode->Info->HorizontalResolution;
    height = m_raw->Mode->Info->VerticalResolution;
}

void graphics_protocol::set_mode(uint32 index)
{
    m_raw->SetMode(m_raw.get(), index);
}

void graphics_protocol::draw_pixel(uint32 x, uint32 y, uint8 r, uint8 g,
                                   uint8 b)
{
    EFI_GRAPHICS_OUTPUT_BLT_PIXEL pixel;
    pixel.Red = r;
    pixel.Green = g;
    pixel.Blue = b;
    pixel.Reserved = 0;

    m_raw->Blt(m_raw.get(), &pixel, EfiBltBufferToVideo, 0, 0, x, y, 1, 1, 0);
}

graphics_protocol::raw* graphics_protocol::get_raw()
{
    return m_raw.get();
}

mbi::frame_buffer graphics_protocol::frame_buffer()
{
    return mbi::frame_buffer{(void*)m_raw->Mode->FrameBufferBase,
                             (uint64)m_raw->Mode->Info->HorizontalResolution,
                             (uint64)m_raw->Mode->Info->VerticalResolution};
}