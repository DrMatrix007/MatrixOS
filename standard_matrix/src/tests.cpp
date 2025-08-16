#include "tuple.hpp"
#include "variant.hpp"

using namespace mst;

constexpr tuple<int, int, int> data{1, 2, 3};

static_assert(data.get<0>() == 1);
static_assert(data.get<1>() == 2);
static_assert(data.get<2>() == 3);

static_assert(is_unique_tuple_v<uint16>);
static_assert(is_unique_tuple_v<uint16, uint32>);
static_assert(!is_unique_tuple_v<uint16, uint32, uint16>);
