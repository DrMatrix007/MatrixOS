#if !defined(STANDARD_MATRIX_TYPE_TRAIT_SAME_H)
#define STANDARD_MATRIX_TYPE_TRAIT_SAME_H


#include "int_types.hpp"
namespace mst
{
       /// is same
    template <typename A, typename B>
    class is_same
    {
    public:
        static constexpr bool value = false;
    };
    template <typename A>
    class is_same<A, A>
    {
    public:
        static constexpr bool value = true;
    };

    template <typename A, typename B>
    constexpr bool is_same_v = is_same<A, B>::value;

    template <typename A, typename B>
    concept same_as = is_same_v<A, B>;
};


#endif // STANDARD_MATRIX_TYPE_TRAIT_SAME_H
