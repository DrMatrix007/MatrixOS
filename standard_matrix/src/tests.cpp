#include "tuple.hpp"
#include "type_traits/is_unique_tuple.hpp"

namespace tests
{
    using namespace mst;

    constexpr tuple<int, int, int> data{1, 2, 3};

    static_assert(data.get<0>() == 1, "Tuple element 0 must be 1");
    static_assert(data.get<1>() == 2, "Tuple element 1 must be 2");
    static_assert(data.get<2>() == 3, "Tuple element 2 must be 3");

    static_assert(is_unique_tuple_v<uint16>, "Single type should be unique");
    static_assert(is_unique_tuple_v<uint16, uint32>, "Different types should be unique");
    static_assert(!is_unique_tuple_v<uint16, uint32, uint16>, "Duplicate types are not unique");
}