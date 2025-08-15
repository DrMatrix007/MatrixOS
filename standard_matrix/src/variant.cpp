#include "variant.hpp"

using namespace mst;


class B
{
public:
    constexpr B(int& x, int &r) : m_y(r), m_x(x) {}
    constexpr B(B &&b) : m_y(b.m_y), m_x(b.m_x)
    {
    };
    constexpr B(const B &) = delete;
    constexpr ~B()
    {
        m_y = m_x;
    }
    int& m_x;
    int& m_y;
};
class C
{
};

constexpr bool test()
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

static_assert(test());