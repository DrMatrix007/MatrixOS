#ifndef STANDARD_MATRIX_RESULT_H
#define STANDARD_MATRIX_RESULT_H

#include "ref.hpp"
#include "variant.hpp"
namespace mst
{
template <typename value, typename error>
class result : public variant<value, error>
{

public:
    template <typename t>
    result(t&& val) : variant<value, error>(mst::move(val))
    {
    }
    template <typename t>
    result(const_ref<t> val) : variant<value, error>(mst::move(val))
    {
    }
    result(const result&) = default;
    result& operator=(const result&) = default; 
    result(result&&) = default;
    result& operator=(result&&) = default;

    template <typename type = value> constexpr variant_view<type> view()
    {
        return variant<value, error>::template view<type>();
    }
    template <typename type = value>
    constexpr variant_view<const type> view() const
    {
        return variant<value, error>::template view<type>();
    }
    constexpr bool has_value() const
    {
        return variant<value, error>::template try_get<value>() != nullptr;
    }
    constexpr bool has_error()
    {
        return !has_value();
    }
    constexpr explicit operator bool() const
    {
        return has_value();
    }
};
} // namespace mst

#endif // STANDARD_MATRIX_RESULT_H