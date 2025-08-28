#ifndef STANDARD_MATRIX_INT_TYPES_H
#define STANDARD_MATRIX_INT_TYPES_H

using int8 = signed char;
using int16 = short;
using int32 = int;
using int64 = long long;

using uint8 = unsigned char;
using uint16 = unsigned short;
using uint32 = unsigned int;
using uint64 = unsigned long long;

static_assert(sizeof(int8) == 1, "int8 must be 8 bits");
static_assert(sizeof(int16) == 2, "int16 must be 16 bits");
static_assert(sizeof(int32) == 4, "int32 must be 32 bits");
static_assert(sizeof(int64) == 8, "int64 must be 64 bits");

static_assert(sizeof(uint8) == 1, "uint8 must be 8 bits");
static_assert(sizeof(uint16) == 2, "uint16 must be 16 bits");
static_assert(sizeof(uint32) == 4, "uint32 must be 32 bits");
static_assert(sizeof(uint64) == 8, "uint64 must be 64 bits");


#if __has_include("efi.h")
#include "efi.h"
using uintn = UINTN;
using intn = INTN;
#else

#endif

#endif // STANDARD_MATRIX_INT_TYPES_H
