#ifndef STANDARD_MATRIX_MATCH_H
#define STANDARD_MATRIX_MATCH_H

#define _MATCH_NARGS_IMPL(_1, _2, _3, N, ...) N
#define _MATCH_NARGS(...) _MATCH_NARGS_IMPL(__VA_ARGS__, 3, 2, 1)

#define _MATCH_CAT(a, b) a##b
#define _MATCH_DISPATCH(N, ...) _MATCH_CAT(_MATCH_IMPL_, N)(__VA_ARGS__)

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

#endif // STANDARD_MATRIX_MATCH_H