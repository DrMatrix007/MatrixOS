#include "boot_info.hpp"

namespace mbi
{
    boot_info::boot_info(frame_buffer buffer) : m_buffer(buffer)
    {
        
    }

    static_assert(sizeof(boot_info) == 24);
}