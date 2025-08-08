#include "variant.hpp"

using namespace mst;

class TestBase
{
public:
    TestBase(const TestBase &) = delete;
    TestBase(TestBase &&) = default;
    constexpr TestBase(int &r) : m_r(r)
    {
    }
    int &m_r;
};

class A : public TestBase
{
public:
    constexpr A(int &ref) : TestBase(ref) {}
    constexpr A(A &&a) = default;
    constexpr ~A()
    {
        // m_r = 1;
    }
};
class B : public TestBase
{
public:
    constexpr B(int x, int &r) : TestBase(r), m_x(x) {}
    constexpr B(B &&b) : TestBase(b.m_r)
    {
        m_x = b.m_x;
        b.m_x = 0;
    };
    constexpr B(const B &) = delete;
    constexpr ~B()
    {
        m_r = m_x;
    }
    int m_x;
};
class C
{
};

constexpr bool test()
{
    int y = 0;
    int x = 0;
    {
        variant<A, B, C> data = variant<A, B, C>::from<B>(move(B{3, x}));
        if (x == 3)
        {
            return false;
        }
        // variant<A, B> data1(move(data));
    }
    return x == 0;
}

static_assert(test());
