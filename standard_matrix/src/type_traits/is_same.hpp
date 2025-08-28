#ifndef STANDARD_MATRIX_TYPE_TRAITS_SAME_H
#define STANDARD_MATRIX_TYPE_TRAITS_SAME_H


namespace mst
{
    /// is_same
    template <typename type1, typename type2>
    class is_same
    {
    public:
        static constexpr bool value = false;
    };

    template <typename type1>
    class is_same<type1, type1>
    {
    public:
        static constexpr bool value = true;
    };

    template <typename type1, typename type2>
    constexpr bool is_same_v = is_same<type1, type2>::value;

    template <typename type1, typename type2>
    concept same_as = is_same_v<type1, type2>;

    template <typename type1, typename type2>
    concept not_same_as = !is_same_v<type1, type2>;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_SAME_H
