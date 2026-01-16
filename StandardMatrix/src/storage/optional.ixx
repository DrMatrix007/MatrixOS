export module mstd.optional;

import mstd.custom_new;
import mstd.semantics;

namespace mstd
{
    export template <typename T>
    class optional
    {
    public:
        constexpr optional(T&& t);
        constexpr optional(const T& t);

    private:
        union
        {
            T _data;
            char _dummy;
        };
    };

    template <typename T>
    constexpr optional<T>::optional(T&& t)
    {
        new(&_data) T(mstd::move(t));
    }

    template <typename T>
    constexpr optional<T>::optional(const T& t)
    {
        new(&_data) T(t);
    }
}
