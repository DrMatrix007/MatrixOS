#if !defined(STANDARD_MATRIX_TUPLE_H)
#define STANDARD_MATRIX_TUPLE_H

#include "int_types.hpp"

namespace mst
{

    template <typename... Ts>
    class tuple
    {
    };

    template <typename first, typename... rest>
    class tuple<first, rest...>
    {
    public:
        constexpr tuple() = default;
        constexpr tuple(first first_val, rest... rest_val);

        template <uint64 I>
        constexpr const auto &get() const;

        template <uint64 I>
        constexpr auto &get();

    private:
        first m_first;
        tuple<rest...> m_rest;
    };

    template <typename first, typename... rest>
    constexpr inline tuple<first, rest...>::tuple(first first_val, rest... rest_val) : m_first(first_val), m_rest(rest_val...)
    {
    }

    template <typename first, typename... rest>
    template <uint64 I>
    constexpr inline const auto &tuple<first, rest...>::get() const
    {

        if constexpr (I == 0)
        {
            return m_first;
        }
        else
        {
            return m_rest.template get<I - 1>();
        }
    }
    template <typename first, typename... rest>
    template <uint64 I>
    constexpr inline auto &tuple<first, rest...>::get()
    {
        if constexpr (I == 0)
        {
            return m_first;
        }
        else
        {
            return m_rest.template get<I - 1>();
        }
    }

}

#endif // STANDARD_MATRIX_TUPLE_H
