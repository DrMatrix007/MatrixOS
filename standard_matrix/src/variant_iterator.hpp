#ifndef MST_VARIANT_ITERATOR_H
#define MST_VARIANT_ITERATOR_H

#include "type_traits.hpp"

namespace mst
{

    template <typename t>
    class variant_value_iterator
    {
    private:
        mst::remove_reference_t<t> *m_ptr;

    public:
        constexpr variant_value_iterator(mst::remove_reference_t<t> *p) : m_ptr(p) {}

        constexpr mst::remove_reference_t<t> &operator*() const { return *m_ptr; }
        constexpr mst::remove_reference_t<t> *operator->() const { return m_ptr; }

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
    class variant_view
    {
    private:
        mst::remove_reference_t<t> *m_ptr;

    public:
        constexpr variant_view(mst::remove_reference_t<t>* p) : m_ptr(p) {}
        // constexpr variant_view(ref<t> p) : m_ptr(p.get()) {}
        // constexpr variant_view(const_ref<t> p) : m_ptr(&*p) {}

        constexpr auto begin() { return variant_value_iterator<t>(m_ptr); }
        constexpr auto end() { return variant_value_iterator<t>(nullptr); }

        constexpr auto begin() const { return variant_value_iterator<const t>(m_ptr); }
        constexpr auto end() const { return variant_value_iterator<const t>(nullptr); }

        constexpr bool is_empty() const { return m_ptr == nullptr; }
    };

    // template <is_ref t>
    // class variant_view<t>
    // {
    // private:
    //     mst::remove_reference_t<t> *m_ptr;

    // public:
    //     constexpr variant_view(t p) : m_ptr(&p) {}

    //     constexpr auto begin() { return variant_value_iterator<t>(m_ptr); }
    //     constexpr auto end() { return variant_value_iterator<t>(nullptr); }

    //     constexpr auto begin() const { return variant_value_iterator<const t>(m_ptr); }
    //     constexpr auto end() const { return variant_value_iterator<const t>(nullptr); }

    //     constexpr bool is_empty() const { return m_ptr == nullptr; }
    // };

}

#endif // MST_VARIANT_ITERATOR_H
