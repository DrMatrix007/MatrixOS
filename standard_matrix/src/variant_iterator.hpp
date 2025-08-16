#ifndef MST_VARIANT_ITERATOR_H
#define MST_VARIANT_ITERATOR_H

namespace mst
{
    template <typename t>
    struct variant_value_iterator
    {
        t *m_ptr;

        constexpr variant_value_iterator(t *p) : m_ptr(p) {}

        constexpr t &operator*() const { return *m_ptr; }
        constexpr t *operator->() const { return m_ptr; }

        constexpr variant_value_iterator &operator++()
        {
            m_ptr = nullptr;
            return *this;
        }

        constexpr bool operator!=(const variant_value_iterator &other) const
        {
            return m_ptr != other.m_ptr;
        }
    };

    template <typename t>
    struct variant_view
    {
        t *m_ptr;

        constexpr variant_view(t *p) : m_ptr(p) {}

        constexpr auto begin() { return variant_value_iterator<t>(m_ptr); }
        constexpr auto end() { return variant_value_iterator<t>(nullptr); }

        constexpr auto begin() const { return variant_value_iterator<const t>(m_ptr); }
        constexpr auto end() const { return variant_value_iterator<const t>(nullptr); }

        constexpr bool is_empty() const { return m_ptr == nullptr; }
    };
}

#endif // MST_VARIANT_ITERATOR_H
