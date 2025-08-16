#ifndef STANDARD_MATRIX_TYPE_TRAITS_TYPE_IN_GROUP_H
#define STANDARD_MATRIX_TYPE_TRAITS_TYPE_IN_GROUP_H

#include "is_same.hpp"

namespace mst
{
    template <typename... types>
    class type_in_group
    {
    };

    template <typename type, typename... types>
    constexpr bool type_in_group_v = type_in_group<type, types...>::value;

    template <typename type, typename first, typename... rest>
    class type_in_group<type, first, rest...>
    {
    public:
        static constexpr bool value = is_same_v<type, first> || type_in_group_v<type, rest...>;
    };

    template <typename type>
    class type_in_group<type>
    {
    public:
        static constexpr bool value = false;
    };

    template <typename type, typename... types>
    concept in_group = type_in_group_v<type, types...>;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_TYPE_IN_GROUP_H
