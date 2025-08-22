#if !defined(STANDARD_MATRIX_TYPE_TRAITS_ENABLE_IF_H)
#define STANDARD_MATRIX_TYPE_TRAITS_ENABLE_IF_H

namespace mst
{
    template <bool Condition, typename T = void>
    struct enable_if
    {
    };

    template <typename T>
    struct enable_if<true, T>
    {
        using type = T;
    };
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_ENABLE_IF_H
