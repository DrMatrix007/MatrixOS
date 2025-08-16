#ifndef STANDARD_MATRIX_OPTIONAL_H
#define STANDARD_MATRIX_OPTIONAL_H

#include "match.hpp"
#include "variant.hpp"

namespace mst
{
    class nullopt
    {
    };

    template <typename type>
        requires(!same_as<type, nullopt>)
    class optional
    {
    public:
        constexpr optional();
        constexpr optional(const type &value);
        constexpr optional(type &&value);

        optional(const optional &) = default;
        optional(optional &&) = default;

        optional &operator=(const optional &) = default;
        optional &operator=(optional &&) = default;

        constexpr bool has_value() const;
        constexpr bool is_empty();
        constexpr type *try_get();
        constexpr const type *try_get() const;

        void reset();
        template <typename... Args>
        type &emplace(Args &&...args);

        constexpr variant_view<type> view();
        constexpr variant_view<const type> view() const;

        constexpr explicit operator bool() const;

    private:
        variant<type, nullopt> m_variant;
    };

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr optional<type>::optional() : m_variant(nullopt{}) {}

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr optional<type>::optional(const type &value) : m_variant(value) {}

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr optional<type>::optional(type &&value) : m_variant(move(value)) {}

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr bool optional<type>::has_value() const
    {
        return m_variant.template try_get<type>() != nullptr;
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    inline constexpr bool optional<type>::is_empty()
    {
        return !has_value();
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr type *optional<type>::try_get()
    {
        return m_variant.template try_get<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr const type *optional<type>::try_get() const
    {
        return m_variant.template try_get<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    void optional<type>::reset()
    {
        m_variant = variant<type, nullopt>(nullopt{});
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    template <typename... Args>
    type &optional<type>::emplace(Args &&...args)
    {
        m_variant = variant<type, nullopt>(type(args...));
        return *m_variant.template try_get<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    inline constexpr variant_view<type> optional<type>::view()
    {
        return m_variant.template view<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    inline constexpr variant_view<const type> optional<type>::view() const
    {
        return m_variant.template view<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt>)
    constexpr optional<type>::operator bool() const
    {
        return has_value();
    }
}

#endif // STANDARD_MATRIX_OPTIONAL_H
