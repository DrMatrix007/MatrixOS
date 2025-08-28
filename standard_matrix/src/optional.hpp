#ifndef STANDARD_MATRIX_OPTIONAL_H
#define STANDARD_MATRIX_OPTIONAL_H

#include "match.hpp"
#include "variant.hpp"

namespace mst
{
class nullopt_t
{
};

constexpr nullopt_t nullopt{};

template <typename type> class optional
{
};

template <typename type>
    requires(!same_as<type, nullopt_t>)
class optional<type>
{
public:
    constexpr optional() : m_variant(nullopt_t{})
    {
    }
    explicit constexpr optional(const_ref<type> value) : m_variant(value)
    {
    }
    constexpr optional(type&& value) : m_variant(mst::forward<type>(value))
    {
    }
    constexpr optional(const nullopt_t&) : m_variant(nullopt_t{})
    {
    }

    optional(const optional&) = default;
    optional(optional&&) = default;
    optional& operator=(const optional&) = default;
    optional& operator=(optional&&) = default;

    static optional from_const_ref(const type& ref)
    {
        return optional(const_ref(ref));
    }

    constexpr bool has_value() const
    {
        return m_variant.template try_get<type>() != nullptr;
    }
    constexpr bool is_empty()
    {
        return !has_value();
    }

    constexpr ref<type> try_get()
    {
        type& ref_val = *m_variant.template try_get<type>();
        return ref<type>(ref_val);
    }
    constexpr const_ref<type> try_get() const
    {
        type& ref_val = *m_variant.template try_get<type>();
        return const_ref<type>(ref_val);
    }

    void reset()
    {
        m_variant = variant<type, nullopt_t>(nullopt);
    }

    template <typename... Args> type& emplace(Args&&... args)
    {
        m_variant = variant<type, nullopt_t>(type(args...));
        return *m_variant.template try_get<type>();
    }

    constexpr variant_view<type> view()
    {
        return m_variant.template view<type>();
    }
    constexpr variant_view<const type> view() const
    {
        return m_variant.template view<type>();
    }

    constexpr explicit operator bool() const
    {
        return has_value();
    }

    constexpr optional& operator=(const_ref<type> value)
    {
        m_variant = variant<type, nullopt_t>(value);
        return *this;
    }
    constexpr optional& operator=(type value)
    {
        m_variant = variant<type, nullopt_t>(move(value));
        return *this;
    }
    constexpr optional& operator=(nullopt_t) noexcept
    {
        m_variant = variant<type, nullopt_t>(nullopt);
        return *this;
    }

    constexpr optional<type&> as_ref()
    {
        match(value, *this)
        {
            return value;            
        }
        return nullopt;
    }

private:
    variant<type, nullopt_t> m_variant;
};
} // namespace mst

#endif // STANDARD_MATRIX_OPTIONAL_H
