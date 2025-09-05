#ifndef MATRIX_BOOT_INFO_BOOT_INFO_H
#define MATRIX_BOOT_INFO_BOOT_INFO_H

#include "graphics/frame_buffer.hpp"

namespace mbi
{
    
    struct __attribute__((packed)) boot_info
    {
        boot_info(frame_buffer buffer);

        frame_buffer m_buffer;
    };
}

#endif // MATRIX_BOOT_INFO_BOOT_INFO_H