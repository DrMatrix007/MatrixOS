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
        variant<int16, B, uint64> data(B{3, x});
        if (x == 3)
        {
            return false;
        }
        variant<int16, B, uint64> data1 = move(data);
        if (x == 3)
        {
            return false;
        }
        variant<int16, B, uint64> data2(move(data1));
        if (x == 3)
        {
            return false;
        }
        variant<int16, B, uint64> data3 = (int16)10;
        data3 = move(data2);
        if (x == 3)
        {
            return false;
        }
    }
    return x == 3;
}

static_assert(test());