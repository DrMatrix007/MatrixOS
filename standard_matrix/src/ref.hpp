#ifndef STANDARD_MATRIX_REF_H
#define STANDARD_MATRIX_REF_H

#include "type_traits/remove_reference.hpp"

namespace mst
{
    template <typename holder>
    class ref
    {
    private:
        using type = mst::remove_reference_t<holder>;

    public:
        constexpr ref(type &ref) : m_ptr(&ref) {}

        constexpr type *get() { return m_ptr; }
        constexpr const type *get() const { return m_ptr; }

        constexpr type *operator->() { return m_ptr; }
        constexpr const type *operator->() const { return m_ptr; }

        constexpr type &operator*() { return *m_ptr; }
        constexpr const type &operator*() const { return *m_ptr; }

        constexpr operator type &() { return *m_ptr; }
        constexpr operator const type &() const { return *m_ptr; }

        constexpr operator type *() { return m_ptr; }
        constexpr operator const type *() const { return m_ptr; }

        // assignment to underlying object
        constexpr ref &operator=(const type &value)
        {
            *m_ptr = value;
            return *this;
        }
        constexpr ref &operator=(type &&value)
        {
            *m_ptr = static_cast<type &&>(value);
            return *this;
        }

        // comparison with other ref
        constexpr bool operator==(const ref &other) const { return *m_ptr == *other.m_ptr; }
        constexpr bool operator!=(const ref &other) const { return *m_ptr != *other.m_ptr; }

        // comparison with raw type
        constexpr bool operator==(const type &value) const { return *m_ptr == value; }
        constexpr bool operator!=(const type &value) const { return *m_ptr != value; }

    private:
        type *m_ptr;
    };

    template <typename holder>
    class const_ref
    {
    private:
        using type = mst::remove_reference_t<holder>;

    public:
        constexpr const_ref(const type &ref) : m_ptr(&ref) {}
        constexpr const_ref(const ref<type> &other) : m_ptr(&other.get()) {}

        const type *get() const { return m_ptr; }

        const type *operator->() const { return m_ptr; }
        const type &operator*() const { return *m_ptr; }

        constexpr operator const type *() const { return m_ptr; }

        // comparison with other const_ref
        constexpr bool operator==(const const_ref &other) const { return *m_ptr == *other.m_ptr; }
        constexpr bool operator!=(const const_ref &other) const { return *m_ptr != *other.m_ptr; }

        // comparison with raw type
        constexpr bool operator==(const type &value) const { return *m_ptr == value; }
        constexpr bool operator!=(const type &value) const { return *m_ptr != value; }

    private:
        const type *m_ptr;
    };

    template <typename t>
    struct ref_warp
    {
        using type = t;
    };
    template <typename t>
    struct ref_warp<t &>
    {
        using type = ref<t>;
    };
    template <typename t>
    struct ref_warp<const t &>
    {
        using type = const_ref<t>;
    };

    template <typename t>
    using ref_wrap_t = typename ref_warp<t>::type;

    template <typename t>
    constexpr ref<t> ref_of(t &reference)
    {
        return ref<t>(reference);
    }
    template <typename t>
    constexpr const_ref<t> ref_of(const t &reference)
    {
        return const_ref<t>(reference);
    }

    template <typename t>
    struct wrap_ref_s
    {
        using type = t;
    };

    template <typename t>
    struct wrap_ref_s<t &>
    {
        using type = ref<t>;
    };

    template <typename t>
    struct wrap_ref_s<const t &>
    {
        using type = const_ref<t>;
    };

    template <typename type>
    using wrap_ref_t = typename wrap_ref_s<type>::type;

}

#endif // STANDARD_MATRIX_REF_H
