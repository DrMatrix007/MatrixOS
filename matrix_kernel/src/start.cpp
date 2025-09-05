// #include "multiboot_header.hpp"
// #include "kernel.hpp"
#include "boot_info.hpp"
#include "graphics/frame_buffer.hpp"

extern "C"
{
    void _start(mbi::boot_info* info)
    {
        for (int x = 0; x < info->m_buffer.width(); x++)
        {
            for (int y = 0; y < info->m_buffer.height(); y++)
            {
                info->m_buffer.set_pixel(x, y, mbi::pixel{69,69,69,69});
            }
        }
    }
}