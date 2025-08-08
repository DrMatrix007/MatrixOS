#if !defined(STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFRENCE_H)
#define STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFRENCE_H


#include "int_types.hpp"
namespace mst
{
    /// remove_refrence
    template <typename T>
    struct remove_reference
    {
        using type = T;
    };

    template <typename T>
    struct remove_reference<T &>
    {
        using type = T;
    };

    template <typename T>
    struct remove_reference<T &&>
    {
        using type = T;
    };

    template <typename T>
    using remove_reference_t = typename remove_reference<T>::type;
}


#endif // STANDARD_MATRIX_TYPE_TRAITS_REMOVE_REFRENCE_H
