#ifndef STANDARD_MATRIX_MATCH_H
#define STANDARD_MATRIX_MATCH_H

#define _MATCH_ARG_COUNT(_1,_2,_3,COUNT,...) COUNT
#define _MATCH_COUNT(...) _MATCH_ARG_COUNT(__VA_ARGS__, 3, 2, 1)

#define _MATCH_CONCAT(a,b) a##b
#define _MATCH_EXPAND_CONCAT(a,b) _MATCH_CONCAT(a,b)

#define match(...) _MATCH_EXPAND_CONCAT(_MATCH_DISPATCH_, _MATCH_COUNT(__VA_ARGS__))(__VA_ARGS__)

#define _MATCH_DISPATCH_3(TYPE, VAR_NAME, DATA) \
    if (!(DATA.view<TYPE>()).is_empty()) \
        for (auto&& VAR_NAME : (DATA).view<TYPE>())

#define _MATCH_DISPATCH_2(VAR_NAME, DATA) \
    if (!(DATA.view()).is_empty()) \
        for (auto&& VAR_NAME : (DATA).view())

#endif // STANDARD_MATRIX_MATCH_H
