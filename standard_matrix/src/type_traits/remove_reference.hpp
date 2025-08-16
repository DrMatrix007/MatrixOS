#ifndef STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFERENCE_H
#define STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFERENCE_H

#include "int_types.hpp"

namespace mst
{
    template <typename t>
    struct remove_reference
    {
        using type = t;
    };

    template <typename t>
    struct remove_reference<t &>
    {
        using type = t;
    };

    template <typename t>
    struct remove_reference<t &&>
    {
        using type = t;
    };

    template <typename t>
    using remove_reference_t = typename remove_reference<t>::type;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFERENCE_H
