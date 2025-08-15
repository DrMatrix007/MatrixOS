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
        constexpr variant_storage();
        template <in_group<first, rest...> type>
        constexpr variant_storage(type &&head);
        constexpr variant_storage(variant_storage &&other, int64 index);

        constexpr ~variant_storage() {}
        constexpr void reset();

        template <int64 index>
        constexpr void destruct(int64 target_index);

        template <typename type>
        constexpr type *try_get();

        template <typename type>
        constexpr const type *try_get() const;

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
        constexpr variant_storage(variant_storage &&data, int64 index);
        constexpr ~variant_storage() {}

        template <int64 index>
        constexpr void destruct(int64 target_index);
        constexpr void reset();

        template <typename type>
        constexpr type *try_get();

        template <typename type>
        constexpr const type *try_get() const;

    private:
        first m_head;
        char m_null;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    public:
        constexpr variant(variant<types...> &&other);
        template <typename type>
        constexpr variant(type &&t);

        template <typename type>
            requires(find_type_index_v<type, types...> >= 0)
        static constexpr variant<types...> from(remove_reference_t<type> &&arg);

        constexpr variant &operator=(variant &&other);

        constexpr ~variant();

        template <typename type>
        constexpr const type *try_get() const;
        template <typename type>
        constexpr type *try_get();

    private:
        constexpr void destruct();
        constexpr variant(uint64 index, variant_storage<types...> storage);

        int64 m_index;
        variant_storage<types...> m_storage;
    };

    template <typename type>
    constexpr void destroyObject(type *ptr);

    template <typename first, typename... rest>
    inline constexpr variant_storage<first, rest...>::variant_storage() : m_tail()
    {
    }

    template <typename first>
    inline constexpr variant_storage<first>::variant_storage() : m_null()
    {
    }

    template <typename first, typename... rest>
    template <in_group<first, rest...> type>
    inline constexpr variant_storage<first, rest...>::variant_storage(type &&val)
    {
        if constexpr (same_as<first, remove_reference_t<type>>)
        {
            new (&m_head)(remove_reference_t<type>)(mst::move(val));
        }
        else
        {
            new (&m_tail) variant_storage<rest...>(mst::move(val));
        }
    }

    template <typename first, typename... rest>
    constexpr variant_storage<first, rest...>::variant_storage(variant_storage &&other, int64 index)
    {
        if (index == 0)
        {
            new (&m_head) first(mst::move(other.m_head));
        }
        else
        {
            new (&m_tail) variant_storage<rest...>(move(other.m_tail), index - 1);
        }
    }

    template <typename first>
    constexpr variant_storage<first>::variant_storage(variant_storage &&data, int64 index)
    {
        if (index == 0)
        {
            new (&m_head) first(data.m_head);
        }
    }

    template <typename first>
    inline constexpr variant_storage<first>::variant_storage(first &&head) : m_head(move(head))
    {
    }

    template <typename first, typename... rest>
    inline constexpr void variant_storage<first, rest...>::reset()
    {
        new (&m_tail) variant_storage<rest...>();
    }

    template <typename first>
    inline constexpr void variant_storage<first>::reset()
    {
        new (&m_null) char();
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

    template <typename first, typename... rest>
    template <typename type>
    inline constexpr type *variant_storage<first, rest...>::try_get()
    {
        if constexpr (same_as<first, type>)
        {
            return &m_head;
        }

        return m_tail.template try_get<type>();
    }

    template <typename first, typename... rest>
    template <typename type>
    inline constexpr const type *variant_storage<first, rest...>::try_get() const
    {
        if constexpr (same_as<first, type>)
        {
            return &m_head;
        }

        return m_tail.template try_get<type>();
    }

    template <typename first>
    template <typename type>
    inline constexpr type *variant_storage<first>::try_get()
    {
        if constexpr (same_as<first, type>)
        {
            return &m_head;
        }

        return nullptr;
    }

    template <typename first>
    template <typename type>
    inline constexpr const type *variant_storage<first>::try_get() const
    {
        if (same_as<first, type>)
        {
            return &m_head;
        }

        return nullptr;
    }

    template <typename type>
    template <int64 index>
    constexpr void variant_storage<type>::destruct(int64 target_index)
    {
        if (index == target_index)
        {
            destroyObject(&m_head);
        }
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr variant<types...>::variant(variant<types...> &&other)
    {
        m_index = other.m_index;
        new (&m_storage) variant_storage<types...>(mst::move(other.m_storage), m_index);
        other.m_index = -1;
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <typename type>
    constexpr variant<types...>::variant(type &&t)
    {
        new (&m_storage) variant_storage<types...>(mst::move(t));
        m_index = find_type_index_v<type, types...>;
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

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr variant<types...> &variant<types...>::operator=(variant &&other)
    {
        destruct();
        m_storage.reset();

        m_index = other.m_index;
        new (&m_storage) variant_storage<types...>(move(other.m_storage), other.m_index);
        other.m_index = -1;

        return *this;
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr variant<types...>::~variant()
    {
        destruct();
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr variant<types...>::variant(uint64 index, variant_storage<types...> storage) : m_storage(mst::move(storage)), m_index(index)
    {
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    inline constexpr void variant<types...>::destruct()
    {
        if (m_index != -1)
        {
            m_storage.template destruct<0>(m_index);
        }
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <typename type>
    constexpr const type *variant<types...>::try_get() const
    {
        if (find_type_index_v<type, types...> == m_index)
        {

            return m_storage.template try_get<type>();
        }
        return nullptr;
    }
    
    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <typename type>
    constexpr type *variant<types...>::try_get()
    {
        if (find_type_index_v<type, types...> == m_index)
        {

            return m_storage.template try_get<type>();
        }
        return nullptr;
    }

    template <typename type>
    constexpr void destroyObject(type *ptr)
    {
        ptr->~type();
    }
}

#endif // STANDARD_MATRIX_VARIANT_H