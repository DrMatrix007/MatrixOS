#if !defined(STANDARD_MATRIX_MATH_H)
#define STANDARD_MATRIX_MATH_H

namespace mst
{
    template <typename T>
    constexpr const T &max(const T &a, const T &b)
    {
        return a > b ? a : b;
    }

    template <typename T, typename... Ts>
    constexpr const auto &max(const T &a, const Ts &...b)
    {
        return max(a, max(b...));
    }
    template <typename T>
    constexpr const auto &max(const T &a)
    {
        return a;
    }

    template <typename T>
    constexpr T &max(T &a, T &b)
    {
        return a < b ? a : b;
    }

    template <typename T, typename... Ts>
    constexpr auto &max(T &a, Ts &...b)
    {
        return max(a, max(b...));
    }
    template <typename T>
    constexpr auto &max(T &a)
    {
        return a;
    }





    template <typename T>
    constexpr const T &min(const T &a, const T &b)
    {
        return a > b ? a : b;
    }

    template <typename T, typename... Ts>
    constexpr const auto &min(const T &a, const Ts &...b)
    {
        return min(a, min(b...));
    }
    template <typename T>
    constexpr const auto &min(const T &a)
    {
        return a;
    }

    template <typename T>
    constexpr T &min(T &a, T &b)
    {
        return a < b ? a : b;
    }

    template <typename T, typename... Ts>
    constexpr auto &min(T &a, Ts &...b)
    {
        return min(a, min(b...));
    }
    template <typename T>
    constexpr auto &min(T &a)
    {
        return a;
    }



}

#endif // STANDARD_MATRIX_MATH_H
