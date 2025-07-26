#include "multiboot_header.h"
#include "kernel.h"

__attribute__((aligned(16), section(".bss")))
uint8_t kernel_stack[16 * 1024];

extern "C" void _start()
{
    // 1. Clear PG (bit 31) in CR0
    asm volatile(
        "mov %%cr0, %%rax\n"
        "and $0x7FFFFFFF, %%rax\n"
        "mov %%rax, %%cr0\n"
        :
        :
        : "rax");
    // 2. Set RSP to top of kernel stack
    asm volatile(
        "mov %0, %%rsp\n"
        :
        : "r"(kernel_stack + sizeof(kernel_stack))
        : "rsp");


    // 3. Enable PAE (bit 5) in CR4
    asm volatile(
        "mov %%cr4, %%rax\n"
        "or $0x20, %%rax\n"
        "mov %%rax, %%cr4\n"
        :
        :
        : "rax");
    return;
    // 4. Enable Long Mode (LME) in MSR 0xC0000080 (IA32_EFER)
    asm volatile(
        "mov $0xC0000080, %%ecx\n"
        "rdmsr\n"
        "bts $8, %%eax\n"
        "wrmsr\n"
        :
        :
        : "rax", "rcx", "rdx");

    // kernel_main();
}