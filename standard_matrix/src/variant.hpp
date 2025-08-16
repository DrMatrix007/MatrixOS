#if !defined(STANDARD_MATRIX_VARIANT_H)
#define STANDARD_MATRIX_VARIANT_H

#include <new>
#include <memory>

#include "int_types.hpp"
#include "math.hpp"
#include "move.hpp"
#include "type_traits.hpp"
#include "mem_utils.hpp"
#include "variant_iterator.hpp"
#include "match.hpp"



namespace mst
{
    template <typename type>
    constexpr void destroyObject(type *ptr);

    template <typename... types>
    union variant_storage
    {
    };

    template <typename first, typename... rest>
    union variant_storage<first, rest...>
    {
    public:
        constexpr variant_storage();

        template <in_group<first, rest...> type>
        constexpr variant_storage(type &&val);

        constexpr variant_storage(variant_storage &&other, int64 index);
        constexpr variant_storage(const variant_storage &other, int64 index);

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
        // When sizeof...(rest) == 0, this becomes a dummy `char` that we never read
        variant_storage<rest...> m_tail;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    public:
        constexpr variant(const variant<types...> &other);
        constexpr variant(variant<types...> &&other);

        template <in_group<types...> type>
        constexpr variant(type &&t);

        template <in_group<types...> type>
        static constexpr variant<types...> from(type &&arg);

        constexpr variant &operator=(variant &&other);

        constexpr ~variant();

        template <typename type>
        constexpr const type *try_get() const;

        template <typename type>
        constexpr type *try_get();

        template <in_group<types...> type>
        constexpr variant_view<type> view();

        template <in_group<types...> type>
        constexpr variant_view<const type> view() const;

    private:
        constexpr void destruct();
        constexpr variant(uint64 index, variant_storage<types...> storage);

        int64 m_index{-1};
        variant_storage<types...> m_storage;
    };

    template <typename first, typename... rest>
    inline constexpr variant_storage<first, rest...>::variant_storage()
    {
        new (&m_tail) variant_storage<rest...>();
    }

    template <typename first, typename... rest>
    template <in_group<first, rest...> type>
    inline constexpr variant_storage<first, rest...>::variant_storage(type &&val)
    {
        if constexpr (same_as<first, remove_reference_t<type>>)
        {
            new (&m_head) remove_reference_t<type>(mst::move(val));
        }
        else if constexpr (sizeof...(rest) > 0)
        {
            new (&m_tail) variant_storage<rest...>(mst::move(val));
        }
    }

    template <typename first, typename... rest>
    inline constexpr variant_storage<first, rest...>::variant_storage(variant_storage &&other, int64 index)
    {
        if (index == 0)
        {
            new (&m_head) first(mst::move(other.m_head));
        }
        else if constexpr (sizeof...(rest) > 0)
        {
            new (&m_tail) variant_storage<rest...>(mst::move(other.m_tail), index - 1);
        }
    }

    template <typename first, typename... rest>
    inline constexpr variant_storage<first, rest...>::variant_storage(const variant_storage &other, int64 index)
    {
        if (index == 0)
        {
            new (&m_head) first(other.m_head);
        }
        else if constexpr (sizeof...(rest) > 0)
        {
            new (&m_tail) variant_storage<rest...>(other.m_tail, index - 1);
        }
    }

    template <typename first, typename... rest>
    inline constexpr void variant_storage<first, rest...>::reset()
    {
        if constexpr (sizeof...(rest) > 0)
        {
            new (&m_tail) variant_storage<rest...>();
        }
    }

    template <typename first, typename... rest>
    template <int64 index>
    inline constexpr void variant_storage<first, rest...>::destruct(int64 target_index)
    {
        if (index == target_index)
        {
            destroyObject(&m_head);
        }
        else if constexpr (sizeof...(rest) > 0)
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
        else if constexpr (sizeof...(rest) > 0)
        {
            return m_tail.template try_get<type>();
        }
        return nullptr;
    }

    template <typename first, typename... rest>
    template <typename type>
    inline constexpr const type *variant_storage<first, rest...>::try_get() const
    {
        if constexpr (same_as<first, type>)
        {
            return &m_head;
        }
        else if constexpr (sizeof...(rest) > 0)
        {
            return m_tail.template try_get<type>();
        }
        return nullptr;
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    inline constexpr variant<types...>::variant(variant<types...> &&other)
    {
        m_index = other.m_index;
        new (&m_storage) variant_storage<types...>(mst::move(other.m_storage), m_index);
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    inline constexpr variant<types...>::variant(const variant<types...> &other)
    {
        m_index = other.m_index;
        new (&m_storage) variant_storage<types...>(other.m_storage, m_index);
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <in_group<types...> type>
    inline constexpr variant<types...>::variant(type &&t)
    {
        new (&m_storage) variant_storage<types...>(mst::move(t));
        m_index = find_type_index_v<type, types...>;
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <in_group<types...> type>
    inline constexpr variant<types...> variant<types...>::from(type &&arg)
    {
        variant<types...> v(mst::move(arg));
        return mst::move(v);
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    inline constexpr variant<types...> &variant<types...>::operator=(variant &&other)
    {
        destruct();
        m_storage.reset();

        m_index = other.m_index;
        new (&m_storage) variant_storage<types...>(mst::move(other.m_storage), other.m_index);

        return *this;
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    inline constexpr variant<types...>::~variant()
    {
        destruct();
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    inline constexpr variant<types...>::variant(uint64 index, variant_storage<types...> storage)
        : m_index(static_cast<int64>(index)), m_storage(mst::move(storage))
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
    inline constexpr const type *variant<types...>::try_get() const
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
    inline constexpr type *variant<types...>::try_get()
    {
        if (find_type_index_v<type, types...> == m_index)
        {
            return m_storage.template try_get<type>();
        }
        return nullptr;
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <in_group<types...> type>
    constexpr variant_view<type> variant<types...>::view()
    {
        return variant_view<type>(m_storage.template try_get<type>());
    }

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <in_group<types...> type>
    constexpr variant_view<const type> variant<types...>::view() const
    {
        return variant_view<const type>(m_storage.template try_get<type>());
    }

    template <typename type>
    inline constexpr void destroyObject(type *ptr)
    {
        ptr->~type();
    }

} // namespace mst

#endif // STANDARD_MATRIX_VARIANT_H
