#if !defined(STANDARD_MATRIX_MOVE_H)
#define STANDARD_MATRIX_MOVE_H

#include "type_traits/remove_refrence.hpp"
#include "type_traits/is_same.hpp"

namespace mst
{
    template <typename type>
    constexpr remove_reference_t<type> &&move(type &&t) noexcept
    {
        return static_cast<remove_reference_t<type> &&>(t);
    }
    template <typename type>
    constexpr type &&forward(remove_reference_t<type> &t) noexcept
    {
        return static_cast<type &&>(t);
    }
    template <typename type>
    constexpr type &&forward(remove_reference_t<type> &&t) noexcept
    {
        static_assert(!is_same_v<remove_reference_t<type>, type>, "bad forward: T must not be an lvalue reference");
        return static_cast<type &&>(t);
    }
}

#endif // STANDARD_MATRIX_MOVE_H
