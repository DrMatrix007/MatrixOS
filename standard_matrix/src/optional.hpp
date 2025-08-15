#if !defined(STANDARD_MATRIX_OPTIONAL_H)
#define STANDARD_MATRIX_OPTIONAL_H

#include "variant.hpp"

namespace mst
{
    class optnull
    {
    };

    template <typename type>
        requires(!same_as<type, optnull>)
    class optional : private variant<type, optnull>
    {
    public:
        constexpr optional();

        optional(optional&&) = default;
        optional& operator=(optional&&) = default;
    };

    template <typename type>
        requires(!same_as<type, optnull>)
    constexpr optional<type>::optional() : variant<type, optnull>(optnull{})
    {
    }

}

#endif // STANDARD_MATRIX_OPTIONAL_H
