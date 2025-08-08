#if !defined(STANDARD_MATRIX_MOVE_H)
#define STANDARD_MATRIX_MOVE_H

#include "type_traits/remove_refrence.hpp"

namespace mst
{
    template <typename T>
    constexpr remove_reference_t<T> &&move(T &&t) noexcept
    {
        return static_cast<remove_reference_t<T> &&>(t);
    }
}

#endif // STANDARD_MATRIX_MOVE_H
