#ifndef MATRIX_EFI_BOOTLOADER_GRAPHICS_FRAME_BUFFER_H
#define MATRIX_EFI_BOOTLOADER_GRAPHICS_FRAME_BUFFER_H

#include "int_types.hpp"

namespace mbi
{
struct __attribute__((packed)) pixel
{
    inline pixel(uint8 red_value, uint8 green_value, uint8 blue_value,
                 uint8 alpha_value)
        : red(red_value), green(green_value), blue(blue_value),
          alpha(alpha_value)
    {
    }

    uint8 red;
    uint8 green;
    uint8 blue;
    uint8 alpha;
};

class frame_buffer
{
public:
    frame_buffer(void* buffer, uint64 width, uint64 height);

    void set_pixel(uint64 x, uint64 y, const pixel& p);
    

    inline uint64 width()
    {
        return m_width;
    }
    inline uint64 height()
    {
        return m_height;
    }

private:
    pixel* m_buffer;
    uint64 m_width;
    uint64 m_height;
};
} // namespace mbi

#endif // MATRIX_EFI_BOOTLOADER_GRAPHICS_FRAME_BUFFER_H