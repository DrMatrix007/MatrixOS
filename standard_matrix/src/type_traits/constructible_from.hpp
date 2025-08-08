#if !defined(STANDARD_MATRIX_TYPE_TRAITS_CONSTRUCTIBLE_FROM_H)
#define STANDARD_MATRIX_TYPE_TRAITS_CONSTRUCTIBLE_FROM_H

#include "same.hpp"
#include "../move.hpp"
namespace mst
{
    template <typename...>
    class tuple;
    /// constructible_from
    template <typename type, typename... args>
    concept constructible_from = requires(args... args_value) {
        { type(move(args_value)...) } -> same_as<type>;
    };

    /// constructable_counter
    /// example: count_constructibles_v<tuple<A, B, C>, args...> is the amount of types (A, B, C) which are constructible from "args..."

    template <typename...>
    struct count_constructibles
    {
    };

    template <typename... types>
    constexpr bool count_constructibles_v = count_constructibles<types...>::count;

    template <typename first, typename... rest, typename... args>
    struct count_constructibles<tuple<first, rest...>, args...>
    {
    public:
        static constexpr uint64 count = (constructible_from<first, args...> ? 1 : 0) + count_constructibles_v<tuple<rest...>, args...>;
    };
    
    template <typename... args>
    struct count_constructibles<tuple<>, args...>
    {
    public:
        static constexpr uint64 count = 0;
    };

    /// constructible_once

    template <typename... types>
    struct constructible_once
    {
    };

    template <typename... args, constructible_from<args...> first, typename... rest>
    struct constructible_once<tuple<first, rest...>, args...>
    {
    public:
        static constexpr uint64 value = count_constructibles_v<tuple<first, rest...>, args...>;
        using type = first;
    };

    template <typename first, typename... rest, typename... args>
        requires(!constructible_from<first, args...>)
    struct constructible_once<tuple<first, rest...>, args...>
    {
    public:
        static constexpr uint64 value = count_constructibles_v<tuple<first, rest...>, args...>;
        using type = constructible_once<tuple<rest...>, args...>::type;
    };

    template <typename... types>
    constexpr bool constructible_once_v = constructible_once<types...>::value;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_CONSTRUCTIBLE_FROM_H
