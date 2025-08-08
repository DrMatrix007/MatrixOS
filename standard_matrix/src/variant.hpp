#if !defined(STANDARD_MATRIX_VARIANT_H)
#define STANDARD_MATRIX_VARIANT_H

#include "int_types.hpp"
#include "math.hpp"
#include "move.hpp"
#include "type_traits.hpp"
#include "mem_utils.hpp"
#include <new>
#include <memory>

namespace mst
{
    template <typename... types>
    union variant_storage;

    template <typename first, typename... rest>
    union variant_storage<first, rest...>
    {
    public:
        template <typename type>
        constexpr static variant_storage<first, rest...> from(remove_reference_t<type> &&t);
        constexpr static variant_storage<first, rest...> from(first &&t);

        constexpr variant_storage(first head);
        constexpr variant_storage(variant_storage &&) {};
        constexpr ~variant_storage() {}

        template <typename T>
        constexpr void init(remove_reference_t<T> &&t);

        template <typename T>
        T *as_ptr();

    private:
        first m_head;
        variant_storage<rest...> m_tail;
    };
    template <typename first>
    union variant_storage<first>
    {
    public:
        constexpr variant_storage(first head);
        constexpr variant_storage(variant_storage &&){};
        constexpr ~variant_storage(){}
        template<typename type>
        constexpr static variant_storage<type> from(first &&t);
        constexpr static variant_storage<first> from(first &&t);



        template <typename T>
        T *as_ptr();

    private:
        first m_head;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    public:
        template <typename type>
            requires(find_type_index_v<type, types...> >= 0)
        static constexpr variant<types...> from(remove_reference_t<type> &&arg);

    private:
        constexpr variant(uint64 index, variant_storage<types...> storage);
        static constexpr uint64 size = max(sizeof(types)...);
        static constexpr uint64 align = max(alignof(types)...);

        constexpr void destruct_value();
        template <uint64 index>
        constexpr void destruct_value_impl();

        variant_storage<types...> m_storage;
        uint64 m_index;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr inline void variant<types...>::destruct_value()
    {
        destruct_value_impl<0>();
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <typename type>
        requires(find_type_index_v<type, types...> >= 0)

    inline constexpr variant<types...> variant<types...>::from(remove_reference_t<type> &&arg)
    {
        variant_storage<types...> storage = move(variant_storage<types...>::template from<type>(move(arg)));
        variant v(find_type_index_v<type, types...>, move(storage));

        return move(v);
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <uint64 index>
    constexpr inline void variant<types...>::destruct_value_impl()
    {
        if constexpr (index < sizeof...(types))
        {
            if (m_index == index)
            {
            }
            destruct_value_impl<index + 1>();
        }
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr variant<types...>::variant(uint64 index, variant_storage<types...> storage) : m_storage(move(storage)), m_index(index)
    {
    }

    template <typename first, typename... rest>
    inline constexpr variant_storage<first, rest...> variant_storage<first, rest...>::from(first &&t)
    {
        return variant_storage<first, rest...>(move(t));
    }



    template <typename first, typename... rest>
    template <typename type>
    inline constexpr variant_storage<first, rest...> variant_storage<first, rest...>::from(remove_reference_t<type> &&t)
    {
        return variant_storage<first, rest...>(variant_storage<rest...>::template from<type>(move(t)));
    }

    template<typename first>
    inline constexpr variant_storage<first> variant_storage<first>::from(first &&t)
    {
        return variant_storage<first>(move(t));
    }

}

#endif // STANDARD_MATRIX_VARIANT_H
