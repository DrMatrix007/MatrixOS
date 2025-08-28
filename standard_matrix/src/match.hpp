#ifndef STANDARD_MATRIX_MATCH_H
#define STANDARD_MATRIX_MATCH_H

#define _MATCH_NARGS_IMPL(_1, _2, _3, _4, N, ...) N
#define _MATCH_NARGS(...) _MATCH_NARGS_IMPL(__VA_ARGS__, 4, 3, 2, 1)

#define _MATCH_CAT(a, b) a##b
#define _MATCH_DISPATCH(N, ...) _MATCH_CAT(_MATCH_IMPL_, N)(__VA_ARGS__)
#define _MATCH_CAT(a, b) a##b
#define _MATCH_XCAT(a, b) _MATCH_CAT(a, b)
#define _MATCH_UNIQUE(base) _MATCH_XCAT(base, __LINE__)

/// use:
/// match(type, var, viewable)
/// match(var, viewable)
#define match(...) _MATCH_DISPATCH(_MATCH_NARGS(__VA_ARGS__), __VA_ARGS__)

#define _MATCH_IMPL_3(TYPE, VAR, DATA)                                         \
    if (auto&& _match_data = (DATA); !(_match_data.view<TYPE>()).is_empty())   \
        for (auto&& VAR : _match_data.view<TYPE>())

#define _MATCH_IMPL_2(VAR, DATA)                                               \
    if (auto&& _match_data = (DATA); !(_match_data.view()).is_empty())         \
        for (auto&& VAR : _match_data.view())

#define _MATCH_OR_DISPATCH(N, ...) _MATCH_CAT(_MATCH_OR_IMPL_, N)(__VA_ARGS__)

#define match_or(...) _MATCH_OR_DISPATCH(_MATCH_NARGS(__VA_ARGS__), __VA_ARGS__)

#define _MATCH_OR_IMPL_4(TYPE, VAR, DATA, ON_EMPTY)                            \
    auto&& _MATCH_UNIQUE(_match_or_data) = (DATA).view<TYPE>();                \
    if (_MATCH_UNIQUE(_match_or_data).is_empty())                              \
    {                                                                          \
        return ([&]() { ON_EMPTY; })();                                        \
    }                                                                          \
    auto&& VAR = *(_MATCH_UNIQUE(_match_or_data).begin())

#define _MATCH_OR_IMPL_3(VAR, DATA, ON_EMPTY)                                  \
    auto&& _MATCH_UNIQUE(_match_or_data) = (DATA).view();                      \
    if (_MATCH_UNIQUE(_match_or_data).is_empty())                              \
    {                                                                          \
        return ([&]() { ON_EMPTY; })();                                        \
    }                                                                          \
    auto&& VAR = *(_MATCH_UNIQUE(_match_or_data).begin())
#endif // STANDARD_MATRIX_MATCH_H