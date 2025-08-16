#if !defined(STANDARD_MATRIX_VARIANT_ITERATOR_H)
#define STANDARD_MATRIX_VARIANT_ITERATOR_H

namespace mst
{

    template <typename T>
    struct variant_value_iterator
    {
        T *ptr;

        constexpr variant_value_iterator(T *p) : ptr(p) {}

        constexpr T &operator*() const { return *ptr; }
        constexpr T *operator->() const { return ptr; }

        constexpr variant_value_iterator &operator++()
        {
            ptr = nullptr; // advance past the single element
            return *this;
        }

        constexpr bool operator!=(const variant_value_iterator &other) const
        {
            return ptr != other.ptr;
        }
    };

    // View over the value of a specific type in the variant
    template <typename T>
    struct variant_view
    {
        T *ptr;

        constexpr variant_view(T *p) : ptr(p) {}

        constexpr auto begin() { return variant_value_iterator<T>(ptr); }
        constexpr auto end() { return variant_value_iterator<T>(nullptr); }

        constexpr auto begin() const { return variant_value_iterator<const T>(ptr); }
        constexpr auto end() const { return variant_value_iterator<const T>(nullptr); }
    };

}

#endif // STANDARD_MATRIX_VARIANT_ITERATOR_H
