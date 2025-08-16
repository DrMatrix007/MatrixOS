#include "variant.hpp"
using namespace mst;

class b
{
public:
    constexpr b(int &x, int &r) : m_y(r), m_x(x) {}
    constexpr b(b &&b) : m_y(b.m_y), m_x(b.m_x) {}
    constexpr b(const b &) = delete;
    constexpr ~b() { m_y = m_x; }
    int &m_y;
    int &m_x;
};

class c
{
};

constexpr bool test_raii_move()
{
    int y = 10;
    int x = 0;
    {
        variant<int16, b, uint64> d1(b{y, x});
        variant<int16, b, uint64> d2 = std::move(d1);
        variant<int16, b, uint64> d3(std::move(d2));
        variant<int16, b, uint64> d4 = (int16)10;
        d4 = std::move(d3);
        d4.try_get<b>()->m_x = 8;
        d4 = (int16)10;
        *d4.try_get<int16>() = 0;
    }
    return x == 8;
}

class a
{
public:
    constexpr a(int &a) : m_a(a), m_active(true) {}
    constexpr a(const a &other) : m_a(other.m_a), m_active(other.m_active) {}
    constexpr a(a &&other) : m_a(other.m_a), m_active(other.m_active) { other.m_active = false; }
    constexpr ~a()
    {
        if (m_active)
            m_a++;
    }
    int &m_a;
    bool m_active;
};

constexpr bool test_raii_copy()
{
    int x = 0;
    {
        variant<a> d1{a{x}};
        variant<a> d2{d1};
    }
    return x == 2;
}

struct d
{
    int value;
    constexpr d(int v) : value(v) {}
    constexpr d(const d &o) : value(o.value) {}
    constexpr d(d &&o) : value(o.value) {}
    constexpr d &operator=(const d &o)
    {
        value = o.value;
        return *this;
    }
};

constexpr bool test_basic()
{
    variant<int, float, d> v1(42);
    if (!v1.try_get<int>() || *v1.try_get<int>() != 42)
        return false;

    variant<int, float, d> v2(3.14f);
    if (!v2.try_get<float>() || *v2.try_get<float>() != 3.14f)
        return false;

    variant<int, float, d> v3(d{99});
    if (!v3.try_get<d>() || v3.try_get<d>()->value != 99)
        return false;

    return true;
}

constexpr bool test_copy_move()
{
    variant<int, d> v1(123);
    variant<int, d> v2(v1);
    if (!v2.try_get<int>() || *v2.try_get<int>() != 123)
        return false;

    variant<int, d> v3(d{55});
    variant<int, d> v4(std::move(v3));
    if (!v4.try_get<d>() || v4.try_get<d>()->value != 55)
        return false;

    v2 = variant<int, d>(d{77});
    if (!v2.try_get<d>() || v2.try_get<d>()->value != 77)
        return false;

    return true;
}

constexpr bool test_from()
{
    auto v = variant<int, float>::from(7);
    return v.try_get<int>() && *v.try_get<int>() == 7;
}

constexpr bool test_reset()
{
    variant<int, d> v(d{88});
    if (!v.try_get<d>())
        return false;
    v = variant<int, d>(5);
    return v.try_get<int>() && *v.try_get<int>() == 5;
}

constexpr bool test_iter()
{
    variant<int, bool> data = 16;

    match(int, val, data)
    {
        if (val != 16)
        {
            return false;
        };
        data = true;
    }

    match(bool, val, data)
    {
        return val;
    }

    return false;
}

constexpr bool test_match()
{
    variant<int, bool> data = true;

    match(bool, value, data)
    {
        if (!value)
        {
            return false;
        }
    }
    else
    {
        return false;
    }

    match(int, val, data)
    {
        return val == 1;
    }
    else match(bool, _, data)
    {
        return true;
    }

    return false;
}

static_assert(test_raii_move());
static_assert(test_raii_copy());
static_assert(test_basic());
static_assert(test_copy_move());
static_assert(test_from());
static_assert(test_reset());
static_assert(test_iter());
static_assert(test_match());
