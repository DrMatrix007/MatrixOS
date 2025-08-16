#ifndef MST_VARIANTS_H
#define MST_VARIANTS_H

#include <new>

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
    constexpr void destroyObject(type *ptr)
    {
        ptr->~type();
    }

    template <typename... types>
    union variant_storage {};

    template <typename first, typename... rest>
    union variant_storage<first, rest...>
    {
    public:
        constexpr variant_storage() { new (&m_tail) variant_storage<rest...>(); }

        template <in_group<first, rest...> type>
        constexpr variant_storage(type &&val)
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

        constexpr variant_storage(variant_storage &&other, int64 index)
        {
            if (index == 0) new (&m_head) first(mst::move(other.m_head));
            else if constexpr (sizeof...(rest) > 0)
                new (&m_tail) variant_storage<rest...>(mst::move(other.m_tail), index - 1);
        }

        constexpr variant_storage(const variant_storage &other, int64 index)
        {
            if (index == 0) new (&m_head) first(other.m_head);
            else if constexpr (sizeof...(rest) > 0)
                new (&m_tail) variant_storage<rest...>(other.m_tail, index - 1);
        }

        constexpr ~variant_storage() {}

        constexpr void reset()
        {
            if constexpr (sizeof...(rest) > 0) new (&m_tail) variant_storage<rest...>();
        }

        template <int64 index>
        constexpr void destruct(int64 target_index)
        {
            if (index == target_index) destroyObject(&m_head);
            else if constexpr (sizeof...(rest) > 0)
                m_tail.template destruct<index + 1>(target_index);
        }

        template <typename type>
        constexpr type *try_get(int64 index)
        {
            if constexpr (same_as<first, type>)
            {
                if (index == 0) return &m_head;
            }
            else if constexpr (sizeof...(rest) > 0)
            {
                return m_tail.template try_get<type>(index - 1);
            }
            return nullptr;
        }

        template <typename type>
        constexpr const type *try_get(int64 index) const
        {
            if constexpr (same_as<first, type>)
            {
                if (index == 0) return &m_head;
            }
            else if constexpr (sizeof...(rest) > 0)
            {
                return m_tail.template try_get<type>(index - 1);
            }
            return nullptr;
        }

    private:
        first m_head;
        variant_storage<rest...> m_tail;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    public:
        constexpr variant(const variant<types...> &other)
        {
            m_index = other.m_index;
            new (&m_storage) variant_storage<types...>(other.m_storage, m_index);
        }

        constexpr variant(variant<types...> &&other)
        {
            m_index = other.m_index;
            new (&m_storage) variant_storage<types...>(mst::move(other.m_storage), m_index);
        }

        template <in_group<types...> type>
        constexpr variant(type &&t)
        {
            new (&m_storage) variant_storage<types...>(mst::move(t));
            m_index = find_type_index_v<type, types...>;
        }

        template <in_group<types...> type>
        static constexpr variant<types...> from(type &&arg)
        {
            variant<types...> v(mst::move(arg));
            return mst::move(v);
        }

        constexpr variant &operator=(variant &&other)
        {
            destruct();
            m_storage.reset();

            m_index = other.m_index;
            new (&m_storage) variant_storage<types...>(mst::move(other.m_storage), other.m_index);

            return *this;
        }

        constexpr ~variant() { destruct(); }

        template <typename type>
        constexpr const type *try_get() const
        {
            if (find_type_index_v<type, types...> == m_index)
            {
                return m_storage.template try_get<type>(m_index);
            }
            return nullptr;
        }

        template <typename type>
        constexpr type *try_get()
        {
            if (find_type_index_v<type, types...> == m_index)
            {
                return m_storage.template try_get<type>(m_index);
            }
            return nullptr;
        }

        template <in_group<types...> type>
        constexpr variant_view<type> view()
        {
            return variant_view<type>(m_storage.template try_get<type>(m_index));
        }

        template <in_group<types...> type>
        constexpr variant_view<const type> view() const
        {
            return variant_view<const type>(m_storage.template try_get<type>(m_index));
        }

    private:
        constexpr void destruct()
        {
            if (m_index != -1) m_storage.template destruct<0>(m_index);
        }

        constexpr variant(uint64 index, variant_storage<types...> storage)
            : m_index(static_cast<int64>(index)), m_storage(mst::move(storage)) {}

        int64 m_index{-1};
        variant_storage<types...> m_storage;
    };
}

#endif // MST_VARIANTS_H
