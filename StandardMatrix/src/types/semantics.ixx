export module mstd.semantics;

import mstd.type_traits;

namespace mstd
{
    export class non_movable
    {
    protected:
        non_movable() = default;
        non_movable(non_movable&&) = delete;
        non_movable& operator=(non_movable&&) = delete;
        ~non_movable() = default;
    };

    export class non_copyable
    {
    protected:
        non_copyable() = default;
        non_copyable(const non_copyable&) = delete;
        non_copyable& operator=(const non_copyable&) = delete;
        ~non_copyable() = default;
    };

    export template <class T>
    constexpr remove_reference<T>&& move(T&& t) noexcept;

    template <class T>
    constexpr remove_reference<T>&& move(T&& t) noexcept
    {
        return static_cast<remove_reference<T>&&>(t);
    }
}
