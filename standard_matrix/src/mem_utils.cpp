#include "mem_utils.hpp"

using namespace mst;

// constexpr void mst::memcpy(void *dest, const void *src, uint64 count)
// {
//     auto d = static_cast<unsigned char *>(dest);
//     auto s = static_cast<const unsigned char *>(src);
//     for (uint64 i = 0; i < count; ++i)
//     {
//         d[i] = s[i];
//     }
// }