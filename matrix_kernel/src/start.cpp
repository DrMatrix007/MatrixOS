// #include "multiboot_header.hpp"
// #include "kernel.hpp"
#include "boot_info.hpp"

static const int val = 42;

extern "C" int _start()
{
    return val;
}