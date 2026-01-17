import mstd.variant;
import mstd.semantics;

using namespace mstd;

struct test_class : non_movable
{
    constexpr test_class(const test_class& other) : _ref(other._ref)
    {
        _ref++;
    }

    constexpr test_class(int& ref) : _ref(ref)
    {
        _ref++;
    }

    constexpr ~test_class()
    {
        _ref--;
    }

public:
    int& _ref;
};

constexpr bool test_counter()
{
    int counter = 0;
    counter++;
    counter--; // for the editor to not lose its mind.
    // it's not important because it's a constexpr function and does not go to compile time

    {
        variant<int, float, test_class> value(test_class{counter});
        for (auto& v: value.template get<test_class>())
        {
            v._ref++;
        }
    }

    return counter == 1;
}

static_assert(test_counter());
