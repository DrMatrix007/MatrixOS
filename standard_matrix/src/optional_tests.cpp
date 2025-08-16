#include "optional.hpp"
#include "match.hpp"

using namespace mst;

constexpr bool test1()
{
    optional<int> data = 10;

    *data.try_get() = 5;

    const optional<int> data1 = data;

    return *data1.try_get() == 5;
}

constexpr bool test2()
{
    optional<int> data = 10;

    match(val, data)
    {
        return val == 10;
    }
    else
    {
        return false;
    }

    return false;
}

static_assert(test1());
static_assert(test2());

