#if !defined(STANDARD_MATRIX_VARIANT_H)
#define STANDARD_MATRIX_VARIANT_H

#include "int_types.hpp"
#include "move.hpp"
#include "variant_iterator.hpp"
#include "ref.hpp"

#include "stddef.h"

// #if __has_include(<new>)
// #include <new>
// #else
// for non std environents
inline constexpr void *operator new(size_t, void *ptr) noexcept { return ptr; }
inline constexpr void *operator new[](size_t, void *ptr) noexcept { return ptr; }
inline constexpr void operator delete(void *, void *) noexcept {}
inline constexpr void operator delete[](void *, void *) noexcept {}
// #endif

namespace mst
{
    template <typename type>
    inline constexpr void destroyObject(type *ptr)
    {
        ptr->~type();
    }

    template <typename... types>
    union variant_storage
    {
    };

    template <typename first, typename... rest>
    union variant_storage<first, rest...>
    {
    public:
        constexpr variant_storage()
        {
            new (&m_tail) variant_storage<rest...>();
        }

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
            if (index == 0)
            {
                new (&m_head) first(mst::move(other.m_head));
            }
            else if constexpr (sizeof...(rest) > 0)
            {
                new (&m_tail) variant_storage<rest...>(mst::move(other.m_tail), index - 1);
            }
        }

        constexpr variant_storage(const variant_storage &other, int64 index)
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

        constexpr ~variant_storage() noexcept {}

        constexpr void reset()
        {
            if constexpr (sizeof...(rest) > 0)
            {
                new (&m_tail) variant_storage<rest...>();
            }
        }

        template <int64 index>
        constexpr void destruct(int64 target_index) noexcept
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

        template <typename type>
        constexpr type *try_get(int64 index)
        {
            if constexpr (same_as<first, type>)
            {
                if (index == 0)
                {
                    return ref<type>(m_head);
                }
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
                if (index == 0)
                {
                    return &m_head;
                }
            }
            else if constexpr (sizeof...(rest) > 0)
            {
                return m_tail.template try_get<type>(index - 1);
            }
            return nullptr;
        }

    private:
        ref_wrap_t<first> m_head;
        variant_storage<rest...> m_tail;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    private:
    public:
        using storage = variant_storage<wrap_ref_t<types>...>;
        template <in_group<types...> type>
        constexpr variant(type &&t)
        {
            if constexpr (is_ref<type>)
            {
                new (&m_storage) storage(ref_of(t));
            }
            else
            {
                new (&m_storage) storage(mst::move(t));
            }
            m_index = find_type_index_v<type, types...>;
        }

        constexpr variant(const variant<types...> &other)
        {
            m_index = other.m_index;
            new (&m_storage) storage(other.m_storage, m_index);
        }

        constexpr variant(variant<types...> &&other)
        {
            m_index = other.m_index;
            new (&m_storage) storage(mst::move(other.m_storage), m_index);
        }

        constexpr variant &operator=(variant &&other)
        {
            destruct();
            m_storage.reset();

            m_index = other.m_index;
            new (&m_storage) storage(mst::move(other.m_storage), other.m_index);

            return *this;
        }

        constexpr ~variant() noexcept
        {
            destruct();
        }

        template <typename type>
        constexpr remove_reference_t<type> *try_get()
        {
            if (find_type_index_v<type, types...> == m_index)
            {
                if constexpr (is_ref<type>)
                {
                    wrap_ref_t<type> *ptr = m_storage.template try_get<wrap_ref_t<type>>(m_index);
                    if (ptr != nullptr)
                    {
                        return ptr->get();
                    }
                }
                else
                {
                    return m_storage.template try_get<wrap_ref_t<type>>(m_index);
                }
            }
            return nullptr;
        }

        template <typename type>
        constexpr const remove_reference_t<type> *try_get() const
        {
            if (find_type_index_v<type, types...> == m_index)
            {
                if constexpr (is_ref<type>)
                {
                    wrap_ref_t<type> *ptr = m_storage.template try_get<wrap_ref_t<type>>(m_index);
                    if (ptr != nullptr)
                    {
                        return ptr->get();
                    }
                }
                else
                {
                    return m_storage.template try_get<wrap_ref_t<type>>(m_index);
                }
            }
            return nullptr;
        }

        template <in_group<types...> type>
        constexpr variant_view<type> view()
        {
            wrap_ref_t<type> *ptr = m_storage.template try_get<wrap_ref_t<type>>(m_index);
            if (ptr == nullptr)
            {
                return nullptr;
            }
            if constexpr (is_ref<type>)
            {
                return ptr->get();
            }
            else
            {
                return ptr;
            }
        }

        template <in_group<types...> type>
        constexpr variant_view<const type> view() const
        {
            wrap_ref_t<type> *ptr = m_storage.template try_get<wrap_ref_t<type>>(m_index);
            if (ptr == nullptr)
            {
                return nullptr;
            }
            if constexpr (is_ref<type>)
            {
                return ptr->get();
            }
            else
            {
                return ptr;
            }
        }

    private:
        constexpr void destruct() noexcept
        {
            m_storage.template destruct<0>(m_index);
        }

        constexpr variant(uint64 index, variant_storage<types...> storage)
            : m_index(static_cast<int64>(index)), m_storage(mst::move(storage))
        {
        }

        int64 m_index{-1};
        variant_storage<wrap_ref_t<types>...> m_storage;
    };
}; // namespace mst

#endif // STANDARD_MATRIX_VARIANT_H
