export module mstd.type_traits;

namespace mstd
{
    namespace
    {
        template <typename T>
        struct remove_reference_t
        {
            using type = T;
        };

        template <typename T>
        struct remove_reference_t<T&>
        {
            using type = T;
        };

        template <typename T>
        struct remove_reference_t<T&&>
        {
            using type = T;
        };
    }

    export template <typename T>
    using remove_reference = remove_reference_t<T>::type;

    namespace
    {
        template <typename t1, typename t2>
        struct is_same_t
        {
            static constexpr bool value = false;
        };

        template <typename t1>
        struct is_same_t<t1, t1>
        {
            static constexpr bool value = true;
        };
    }

    export template <typename t1, typename t2>
    constexpr bool is_same = is_same_t<t1,t2>::value;

    namespace
    {
        template <typename t, typename... args>
        struct index_of_t;

        template <typename t, typename... rest>
        struct index_of_t<t, t, rest...> {
            static constexpr unsigned int value = 0;
        };

        template <typename t, typename head, typename... rest>
        struct index_of_t<t, head, rest...> {
            static constexpr unsigned int value = 1 + index_of_t<t, rest...>::value;
        };
    }

    export template <typename t, typename head, typename... rest>
    constexpr unsigned int index_of = index_of_t<t,head,rest...>::value;

}
