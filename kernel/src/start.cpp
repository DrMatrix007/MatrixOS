#include "multiboot_header.h"
#include "kernel.h"

__attribute__((aligned(16), section(".bss")))
uint8_t kernel_stack[16 * 1024];

extern "C" void _start()
{
    asm volatile(
        "mov %0, %%esp\n"
        :
        : "r"(kernel_stack + sizeof(kernel_stack)));

    kernel_main();
}