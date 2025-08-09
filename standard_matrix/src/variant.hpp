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
        constexpr static variant_storage<first, rest...> from(type &&t);

        constexpr variant_storage();
        template <not_same_as<variant_storage<first, rest...>> type>
        constexpr variant_storage(type &&head);

        constexpr variant_storage(variant_storage &&other, int64 index)
        {
            if (index == 0)
            {
                new (&m_head) first(mst::move(other.m_head));
            }
            else
            {
                new(&m_tail)variant_storage<rest...>(move(other.m_tail),index-1);
            }
        };
        constexpr ~variant_storage() {}

        constexpr void clear()
        {
            new (&m_tail) variant_storage<rest...>();
        }

        template <typename type>
        type &&take()
        {
            if constexpr (same_as<first, type>)
            {
                return move(m_head);
            }
            else
            {
                return m_tail.template take<type>();
            }
        }

        template <int64 index>
        constexpr void destruct(int64 target_index);

    private:
        first m_head;
        variant_storage<rest...> m_tail;
        char m_null;
    };
    template <typename first>
    union variant_storage<first>
    {
    public:
        constexpr variant_storage();
        constexpr variant_storage(first &&head);
        constexpr variant_storage(variant_storage &&data, int64 index) {
            if(index == 0)
            {
                new(&m_head)first(data.m_head);
            }
        };
        constexpr ~variant_storage() {}

        template <int64 index>
        constexpr void destruct(int64 target_index);
        constexpr void clear()
        {
            new (&m_null) char();
        }

    private:
        first m_head;
        char m_null;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    public:
        constexpr variant(variant<types...> &&other)
        {
            m_index = other.m_index;
            new (&m_storage) variant_storage<types...>(mst::move(other.m_storage), m_index);
            other.m_index = -1;
        }
        template <typename type>
        constexpr variant(type&&t)
        {
            new (&m_storage) variant_storage<types...>(mst::move(t));
            m_index = find_type_index_v<type, types...>;
        }

        template <typename type>
            requires(find_type_index_v<type, types...> >= 0)
        static constexpr variant<types...> from(remove_reference_t<type> &&arg);

        constexpr variant &operator=(variant &&other)
        {
            static_assert(false, "not yet");
            m_storage = move(other.m_storage);
            m_index = other.m_index;

            other.m_storage.clear();
            other.m_index = -1;
            return *this;
        };

        constexpr ~variant();

    private:
        constexpr variant(uint64 index, variant_storage<types...> storage);
        static constexpr uint64 size = max(sizeof(types)...);
        static constexpr uint64 align = max(alignof(types)...);

        constexpr void destruct_value();
        template <uint64 index>
        constexpr void destruct_value_impl();

        variant_storage<types...> m_storage;
        int64 m_index;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr inline void variant<types...>::destruct_value()
    {
        destruct_value_impl<0>();
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
    constexpr variant<types...>::variant(uint64 index, variant_storage<types...> storage) : m_storage(mst::move(storage)), m_index(index)
    {
    }

    template <typename first, typename... rest>
    inline constexpr variant_storage<first, rest...>::variant_storage() : m_tail()
    {
    }
    template <typename first>
    inline constexpr variant_storage<first>::variant_storage() : m_null()
    {
    }

    template <typename first, typename... rest>
    template <not_same_as<variant_storage<first, rest...>> type>
    inline constexpr variant_storage<first, rest...>::variant_storage(type &&val)
    {
        if constexpr (same_as<first, type>)
        {
            new (&m_head)(remove_reference_t<type>)(mst::move(val));
        }
        else
        {
            new (&m_tail) variant_storage<rest...>(mst::move(val));
        }
    }

    template <typename T>
    constexpr void destroyObject(T *ptr)
    {
        ptr->~T(); // Calls the destructor of type T
    }

    template <typename first, typename... rest>
    template <int64 index>
    inline constexpr void variant_storage<first, rest...>::destruct(int64 target_index)
    {
        if (index == target_index)
        {
            destroyObject(&m_head);
        }
        else
        {
            m_tail.template destruct<index + 1>(target_index);
        }
    }

    template <typename type>
    template <int64 index>
    constexpr void variant_storage<type>::destruct(int64 target_index)
    {
        if (target_index == -1)
        {
            return;
        }
        if (index == target_index)
        {
            destroyObject(&m_head);
        }
    }

    template <typename first>
    inline constexpr variant_storage<first>::variant_storage(first &&head) : m_head(move(head))
    {
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr variant<types...>::~variant()
    {

        if (m_index != -1)
        {
            m_storage.template destruct<0>(m_index);
        }
        else
        {
        }
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <typename type>
        requires(find_type_index_v<type, types...> >= 0)

    inline constexpr variant<types...> variant<types...>::from(remove_reference_t<type> &&arg)
    {

        variant<types...> v(find_type_index_v<type, types...>, move(variant_storage<types...>(move(arg))));

        return move(v);
    }

}

#endif // STANDARD_MATRIX_VARIANT_H
