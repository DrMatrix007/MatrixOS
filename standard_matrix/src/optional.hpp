#ifndef STANDARD_MATRIX_OPTIONAL_H
#define STANDARD_MATRIX_OPTIONAL_H

#include "match.hpp"
#include "variant.hpp"

namespace mst
{
    template <typename type>
    class optional
    {
    };

    class nullopt_t
    {
    };

    constexpr nullopt_t nullopt{};

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    class optional<type>
    {
    public:
        constexpr optional();
        explicit constexpr optional(const_ref<type> value);
        constexpr optional(type&& value);
        constexpr optional(const nullopt_t&): m_variant(nullopt_t{}) {}

        optional(const optional &) = default;
        optional(optional &&) = default;

        optional &operator=(const optional &) = default;
        optional &operator=(optional &&) = default;

        constexpr bool has_value() const;
        constexpr bool is_empty();
        constexpr ref<type> try_get();
        constexpr const_ref<type> try_get() const;

        void reset();
        template <typename... Args>
        type &emplace(Args &&...args);

        constexpr variant_view<type> view();
        constexpr variant_view<const type> view() const;

        constexpr explicit operator bool() const;

        constexpr optional &operator=(const_ref<type> value);
        constexpr optional &operator=(type value);
        constexpr optional &operator=(nullopt_t) noexcept;

    private:
        variant<type, nullopt_t> m_variant;
    };

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type>::optional() : m_variant(nullopt_t{})
    {
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type>::optional(const_ref<type> value) : m_variant(value)
    {
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type>::optional(type&& value) : m_variant(mst::forward<type>(value))
    {
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr bool optional<type>::has_value() const
    {
        return m_variant.template try_get<type>() != nullptr;
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    inline constexpr bool optional<type>::is_empty()
    {
        return !has_value();
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr ref<type> optional<type>::try_get()
    {
        type& ref_val = *m_variant.template try_get<type>();
        return ref<type>(ref_val);
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr const_ref<type> optional<type>::try_get() const
    {
        type& ref_val = *m_variant.template try_get<type>();
        return const_ref<type>(ref_val);
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    void optional<type>::reset()
    {
        m_variant = variant<type, nullopt_t>(nullopt);
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    template <typename... Args>
    type &optional<type>::emplace(Args &&...args)
    {
        m_variant = variant<type, nullopt_t>(type(args...));
        return *m_variant.template try_get<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    inline constexpr variant_view<type> optional<type>::view()
    {
        return m_variant.template view<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    inline constexpr variant_view<const type> optional<type>::view() const
    {
        return m_variant.template view<type>();
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type>::operator bool() const
    {
        return has_value();
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type> &optional<type>::operator=(const_ref<type> value)
    {
        m_variant = variant<type, nullopt_t>(value);
        return *this;
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type> &optional<type>::operator=(type value)
    {
        m_variant = variant<type, nullopt_t>(move(value));
        return *this;
    }

    template <typename type>
        requires(!same_as<type, nullopt_t>)
    constexpr optional<type> &optional<type>::operator=(nullopt_t) noexcept
    {
        m_variant = variant<type, nullopt_t>(nullopt);
        return *this;
    }

};

#endif // STANDARD_MATRIX_OPTIONAL_H
