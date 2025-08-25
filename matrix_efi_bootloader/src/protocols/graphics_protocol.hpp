#ifndef MATRIX_EFI_GRAPHICS_PROTOCOL_H
#define MATRIX_EFI_GRAPHICS_PROTOCOL_H

#include "efi.h"
#include "efiprot.h"
#include "optional.hpp"
#include "protocol.hpp"
#include "protocols/simple_output_protocol.hpp"
#include "graphics/frame_buffer.hpp"

namespace matrix_efi
{
class graphics_protocol
{
public:
    using raw = EFI_GRAPHICS_OUTPUT_PROTOCOL;
    static efi_guid guid();

    graphics_protocol(raw* raw);

    mst::optional<int> try_find_mode(uint32 width, uint32 height, mst::optional<simple_output_protocol&> verbose = mst::nullopt);
    void set_mode(uint32 index);
    void get_mode(uint32& width, uint32& height) const;
    void draw_pixel(uint32 x, uint32 y, uint8 r, uint8 g, uint8 b);

    raw* get_raw();

    mbi::frame_buffer frame_buffer();
private:
    raw* m_raw;
};

} // namespace matrix_efi

#endif // MATRIX_EFI_GRAPHICS_PROTOCOL_H