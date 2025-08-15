#if !defined(STANDARD_MATRIX_OPTIONAL_H)
#define STANDARD_MATRIX_OPTIONAL_H

#include "variant.hpp"

namespace mst
{
    class optnull
    {
    };

    template <typename type>
        requires(!same_as<type, optnull>)
    class optional
    {
    public:
        optional(const optional&) = default;
    private:
        variant<type, optnull> m_data;
    }
}

#endif // STANDARD_MATRIX_OPTIONAL_H
