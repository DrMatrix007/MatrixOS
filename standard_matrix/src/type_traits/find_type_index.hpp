#if !defined(STANDARD_MATRIX_TYPE_TRAITS_FIND_TYPE_INDEX_H)
#define STANDARD_MATRIX_TYPE_TRAITS_FIND_TYPE_INDEX_H

#include "int_types.hpp"
namespace mst
{
    /// find_type_index
    template <typename... types>
    class find_type_index
    {
    };

    template <typename... types>
    constexpr uint64 find_type_index_v = find_type_index<types...>::value;

    template <typename type, typename first_types, typename... rest_types>
    class find_type_index<type, first_types, rest_types...>
    {
    public:
        static constexpr uint64 value = 1 + find_type_index_v<type, rest_types...>;
    };

    template <typename type, typename... types>
    class find_type_index<type, type, types...>
    {
    public:
        static constexpr uint64 value = 0;
    };

}

#endif // STANDARD_MATRIX_TYPE_TRAITS_FIND_TYPE_INDEX_H
