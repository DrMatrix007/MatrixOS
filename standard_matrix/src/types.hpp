

namespace detail
{
    template< class T, class U >
    concept SameHelper = std::is_same_v<T, U>;
}
 
template< class T, class U >
concept same_as = detail::SameHelper<T, U> && detail::SameHelper<U, T>;

template<class T, class U>
struct is_same : std::false_type {};
 
template<class T>
struct is_same<T, T> : std::true_type {};