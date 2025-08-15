#include "variant.hpp"

using namespace mst;

class B
{
public:
    constexpr B(int &x, int &r) : m_y(r), m_x(x) {}
    constexpr B(B &&b) : m_y(b.m_y), m_x(b.m_x) {
                         };
    constexpr B(const B &) = delete;
    constexpr ~B()
    {
        m_y = m_x;
    }
    int &m_y;
    int &m_x;
};
class C
{
};

constexpr bool test1()
{
    int y = 10;
    int x = 0;
    {
        variant<int16, B, uint64> data(B{y, x});

        variant<int16, B, uint64> data1 = move(data);

        variant<int16, B, uint64> data2(move(data1));

        variant<int16, B, uint64> data3 = (int16)10;
        data3 = move(data2);

        data3.try_get<B>()->m_x = 8;

        data3 = (int16)10;

        *data3.try_get<int16>() = 0;
    }
    return x == 8;
}

class A
{
public:
    constexpr A(int &a) : m_a(a), m_active(true) {}
    constexpr A(const A &other) : m_a(other.m_a), m_active(other.m_active) {};
    constexpr A(A &&other) : m_a(other.m_a), m_active(other.m_active)
    {
        other.m_active = false;
    }
    constexpr ~A()
    {
        if (m_active)
        {
            m_a++;
        }
    }
    int &m_a;
    bool m_active;
};

constexpr bool
test2()
{
    int x = 0;
    {
        variant<A> data{A{x}};
        variant<A> data1{data};
    }

    return x == 2;
}

static_assert(test1());
static_assert(test2());