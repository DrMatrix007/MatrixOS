#include "optional.hpp"

using namespace mst;

constexpr bool test()
{
    optional<int> data;

    return true;
}


static_assert(test());
