#if !defined(_MATRIX_OS_MULTIBOOT_HEADER)
#define _MATRIX_OS_MULTIBOOT_HEADER

#include <efi.h>

constexpr uint32_t MULTIBOOT_MAGIC    = 0x1BADB002;
constexpr uint32_t MULTIBOOT_FLAG     = (1 << 0) | (1 << 1);
constexpr uint32_t MULTIBOOT_CHECKSUM = -(MULTIBOOT_MAGIC + MULTIBOOT_FLAG);

struct [[gnu::packed]] MultibootHeader {
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
};

__attribute__((section(".multiboot"), aligned(4), used))
static const MultibootHeader multiboot_header = {
    MULTIBOOT_MAGIC,
    MULTIBOOT_FLAG,
    MULTIBOOT_CHECKSUM
};

#endif // _MATRIX_OS_MULTIBOOT_HEADER

