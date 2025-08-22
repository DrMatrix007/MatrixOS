#ifndef STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFERENCE_H
#define STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFERENCE_H

#include "int_types.hpp"
#include "is_same.hpp"

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


    template<typename t>
    concept not_ref = mst::is_same_v<t,mst::remove_reference_t<t>>;
    template<typename t>
    concept is_ref = !mst::is_same_v<t,mst::remove_reference_t<t>>;
    
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFERENCE_H
