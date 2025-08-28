#include "constructible_from.hpp"
#include "int_types.hpp"

using namespace mst;

static_assert(mst::constructible_from<uint16, uint16>);
