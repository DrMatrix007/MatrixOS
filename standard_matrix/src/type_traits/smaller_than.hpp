#ifndef STANDARD_MATRIX_TYPE_TRAITS_SMALLER_THAN_H
#define STANDARD_MATRIX_TYPE_TRAITS_SMALLER_THAN_H

#include "int_types.hpp"

namespace mst
{
    /// smaller_than
    template <typename t, uint64 size, uint64 align>
    concept smaller_than = sizeof(t) <= size && alignof(t) <= align;
}

#endif // STANDARD_MATRIX_TYPE_TRAITS_SMALLER_THAN_H
