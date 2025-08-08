#include "variant.hpp"

using namespace mst;

class TestBase
{
public:
    constexpr TestBase(int &r) : m_r(r)
    {
    }
    int &m_r;
};

class A : public TestBase
{
public:
    ~A()
    {
        m_r = 1;
    }
};
class B : public TestBase
{
public:
    ~B()
    {
        m_r = 2;
    }
};

constexpr bool test()
{
    int v = 0;
    {
        variant<A, B> value(A{v});
    }

    return v == 1;
}

static_assert(test());