#include "frame_buffer.hpp"

mbi::frame_buffer::frame_buffer(void* buffer, uint64 width, uint64 height)
    : m_buffer((pixel*)buffer), m_width(width), m_height(height)
{
}

void mbi::frame_buffer::set_pixel(uint64 x, uint64 y, const pixel& p)
{
    if (x < m_width && y < m_height)
    {
        m_buffer[y * m_width + x] = p;
    }
}
