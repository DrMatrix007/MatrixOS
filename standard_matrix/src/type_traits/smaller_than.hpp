#if !defined(STANDARD_MATRIX_TYPE_TRAITS_SMALLER_THAN_H)
#define STANDARD_MATRIX_TYPE_TRAITS_SMALLER_THAN_H

#include "int_types.hpp"
namespace mst
{

    /// smaller than
    template <typename T, uint64 size, uint64 align>
    concept smaller_than = sizeof(T) <= size && alignof(T) <= align;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_SMALLER_THAN_H
