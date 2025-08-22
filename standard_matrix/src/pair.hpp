#ifndef STANDARD_MATRIX_PAIR_H
#define STANDARD_MATRIX_PAIR_H

#include "tuple.hpp"

namespace mst
{

    template<typename first, typename second>
    class pair : public tuple<first,second>
    {

    };

}

#endif // STANDARD_MATRIX_PAIR_H