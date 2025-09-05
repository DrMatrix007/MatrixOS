#include "frame_buffer.hpp"

namespace mbi
{
frame_buffer::frame_buffer(void* buffer, uint64 width, uint64 height)
    : m_buffer((pixel*)buffer), m_width(width), m_height(height)
{
}

void frame_buffer::set_pixel(uint64 x, uint64 y, const pixel& p)
{
    if (x < m_width && y < m_height)
    {
        m_buffer[y * m_width + x] = p;
    }
}

pixel::pixel(uint8 red_value, uint8 green_value, uint8 blue_value,
             uint8 alpha_value)
    : red(red_value), green(green_value), blue(blue_value), alpha(alpha_value)
{
}

uint64 frame_buffer::width()
{
    return m_width;
}

uint64 frame_buffer::height()
{
    return m_height;
}


static_assert(sizeof(pixel) == 4);
static_assert(sizeof(frame_buffer) == 24);

} // namespace mbi