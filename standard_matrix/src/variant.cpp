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
    constexpr ~A()
    {
        m_r = 1;
    }
};
class B : public TestBase
{
public:
    constexpr ~B()
    {
        m_r = 2;
    }
};

constexpr bool test()
{
    int x = 0;
    {
        variant<A, B> data = variant<A, B>::from<B>(B{x});
    }
    return x==2;
}

static_assert(test());
