#ifndef STANDARD_MATRIX_MATH_H
#define STANDARD_MATRIX_MATH_H

namespace mst
{
    template <typename t>
    constexpr const t &max(const t &a, const t &b)
    {
        return a > b ? a : b;
    }

    template <typename t, typename... Ts>
    constexpr const auto &max(const t &a, const Ts &...b)
    {
        return max(a, max(b...));
    }

    template <typename t>
    constexpr const auto &max(const t &a)
    {
        return a;
    }

    template <typename t>
    constexpr t &max(t &a, t &b)
    {
        return a > b ? a : b;
    }

    template <typename t, typename... Ts>
    constexpr auto &max(t &a, Ts &...b)
    {
        return max(a, max(b...));
    }

    template <typename t>
    constexpr auto &max(t &a)
    {
        return a;
    }

    template <typename t>
    constexpr const t &min(const t &a, const t &b)
    {
        return a < b ? a : b;
    }

    template <typename t, typename... Ts>
    constexpr const auto &min(const t &a, const Ts &...b)
    {
        return min(a, min(b...));
    }

    template <typename t>
    constexpr const auto &min(const t &a)
    {
        return a;
    }

    template <typename t>
    constexpr t &min(t &a, t &b)
    {
        return a < b ? a : b;
    }

    template <typename t, typename... Ts>
    constexpr auto &min(t &a, Ts &...b)
    {
        return min(a, min(b...));
    }

    template <typename t>
    constexpr auto &min(t &a)
    {
        return a;
    }
}

#endif // STANDARD_MATRIX_MATH_H
