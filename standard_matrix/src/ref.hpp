#if !defined(STANDARD_MATRIX_REF_H)
#define STANDARD_MATRIX_REF_H

namespace mst
{

    template <typename type>
    class ref
    {
    public:
        ref(type &ref) : m_ptr(&ref) {}

        type &get() { return *m_ptr; }
        const type &get() const { return *m_ptr; }

        const type *operator->() const { return m_ptr; }
        const type &operator*() const { return *m_ptr; }

        type *operator->() { return m_ptr; }
        type &operator*() { return *m_ptr; }

    private:
        type *m_ptr;
    };

    template <typename type>
    class const_ref
    {
    public:
        const_ref(const type &ref) : m_ptr(&ref) {}
        const_ref(const ref<type> &other) : m_ptr(&other.get()) {}

        const type &get() const { return *m_ptr; }

        const type *operator->() const { return m_ptr; }
        const type &operator*() const { return *m_ptr; }

    private:
        const type *m_ptr;
    };

}

#endif // STANDARD_MATRIX_REF_H
