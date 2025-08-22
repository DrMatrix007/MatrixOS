#if !defined(STANDARD_MATRIX_TYPE_TRAITS_ENABLE_IF_H)
#define STANDARD_MATRIX_TYPE_TRAITS_ENABLE_IF_H

namespace mst
{
    template <bool Condition, typename type = void>
    struct enable_if
    {
    };

    template <typename t>
    struct enable_if<true, t>
    {
        using type = t;
    };
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_ENABLE_IF_H
