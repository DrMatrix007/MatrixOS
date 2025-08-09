#if !defined(STANDARD_MATRIX_REF_H)
#define STANDARD_MATRIX_REF_H

namespace mst
{

    template <typename type>
    class ref
    {
    public:
        ref(type &ref);

    private:
        type *m_ptr;
    }

    template <typename type>
    inline ref<type>::ref(type &ref): m_ptr(&ref)
    {
    }

}

#endif // STANDARD_MATRIX_REF_H
