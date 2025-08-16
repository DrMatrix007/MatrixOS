#ifndef STANDARD_MATRIX_TYPE_TRAITS_TYPE_INDEX_H
#define STANDARD_MATRIX_TYPE_TRAITS_TYPE_INDEX_H

#include "int_types.hpp"

namespace mst
{
    /// type_index
    template <uint64 I, typename first, typename... rest>
    class type_index
    {
    public:
        using type = typename type_index<I - 1, rest...>::type;
    };

    template <typename first, typename... rest>
    class type_index<0, first, rest...>
    {
    public:
        using type = first;
    };

    template <uint64 I, typename... types>
    using type_index_t = typename type_index<I, types...>::type;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_TYPE_INDEX_H
