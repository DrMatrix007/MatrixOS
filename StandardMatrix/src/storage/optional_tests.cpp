import mstd.optional;
import mstd.semantics;

using namespace mstd;

class test_class : non_copyable
{
public:
    constexpr test_class() = default;
    constexpr test_class(test_class&&) noexcept
    {
    };

    constexpr test_class& operator=(test_class&&) noexcept
    {
        return *this;
    }
};


constexpr bool test_constructor()
{
    optional<test_class> a{{}};
    return true;
}


static_assert(test_constructor());
