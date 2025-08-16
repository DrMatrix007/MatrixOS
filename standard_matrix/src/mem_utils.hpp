#ifndef STANDARD_MATRIX_MEM_UTILS_H
#define STANDARD_MATRIX_MEM_UTILS_H

#include "int_types.hpp"

namespace mst
{
    constexpr void memcpy(void *dest, const void *src, uint64 count)
    {
        auto d = static_cast<unsigned char *>(dest);
        auto s = static_cast<const unsigned char *>(src);
        for (uint64 i = 0; i < count; ++i)
        {
            d[i] = s[i];
        }
    }

    template <class To, class From>
    constexpr To bit_cast(const From &src) noexcept
    {
        To dst;
        memcpy(&dst, &src, sizeof(To));
        return dst;
    }
}

#endif // STANDARD_MATRIX_MEM_UTILS_H
