// #include "multiboot_header.hpp"
// #include "kernel.hpp"
#include "boot_info.hpp"
#include "graphics/frame_buffer.hpp"
#include "int_types.hpp"


__attribute__((naked)) inline void hlt()
{
    asm("hlt");
}


extern "C" void _start(mbi::boot_info* info)
{
    for (uint64 x = 0; x < info->m_buffer.width(); x++)
    {
        for (uint64 y = 0; y < info->m_buffer.height(); y++)
        {
            info->m_buffer.set_pixel(x, y, mbi::pixel{69, 69, 69, 69});
        }
    }
    while (true) {
        hlt();   
    }
}