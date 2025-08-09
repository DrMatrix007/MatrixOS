#if !defined(STANDARD_MATRIX_TYPE_TRAITS_IS_UNIQUE_TUPLE)
#define STANDARD_MATRIX_TYPE_TRAITS_IS_UNIQUE_TUPLE

#include "int_types.hpp"
#include "is_same.hpp"
namespace mst
{
    // unique_tuple
    template <typename... types>
    class is_unique_tuple
    {
    };

    template <typename... types>
    constexpr bool is_unique_tuple_v = is_unique_tuple<types...>::is_unique;

    template <typename type>
    class is_unique_tuple<type>
    {
    public:
        static constexpr bool is_unique = true;
    };

    template <typename first, typename second, typename... rest>
    class is_unique_tuple<first, second, rest...>
    {
    public:
        static constexpr bool is_unique = (!is_same_v<first, second>) && is_unique_tuple_v<first, rest...> && is_unique_tuple_v<second, rest...>;
    };
}


#endif // STANDARD_MATRIX_TYPE_TRAITS_IS_UNIQUE_TUPLE
