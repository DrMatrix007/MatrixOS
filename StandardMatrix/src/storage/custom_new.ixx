//
// Created by ofrih on 1/16/26.
//

export module mstd.custom_new;

#if __has_include(<new>)
#include <new>
#else
export constexpr void* operator new(size_t, void* p) noexcept { return p; }
#endif


