#include "optional.hpp"

using namespace mst;

constexpr bool test1()
{
    optional<int> data;

    return true;
}


static_assert(test1());
