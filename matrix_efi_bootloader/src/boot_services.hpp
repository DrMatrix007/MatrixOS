#ifndef MATRIX_EFI_BOOTLOADER_BOOT_SERVICES_H
#define MATRIX_EFI_BOOTLOADER_BOOT_SERVICES_H

#include "efi.h"
#include "efi_error.hpp"
#include "efidef.h"

namespace matrix_efi
{
constexpr uintn page_size = EFI_PAGE_SIZE;

enum class allocate_type : uint32
{
    any_pages = AllocateAnyPages,
    max_address = AllocateMaxAddress,
    address = AllocateAddress,
    max_allocate = MaxAllocateType,
};

// Match EFI_MEMORY_TYPE
enum class memory_type : uint32
{
    reserved = EfiReservedMemoryType,
    loader_code = EfiLoaderCode,
    loader_data = EfiLoaderData,
    boot_services_code = EfiBootServicesCode,
    boot_services_data = EfiBootServicesData,
    runtime_services_code = EfiRuntimeServicesCode,
    runtime_services_data = EfiRuntimeServicesData,
    conventional_memory = EfiConventionalMemory,
    unusable_memory = EfiUnusableMemory,
    acpi_reclaim_memory = EfiACPIReclaimMemory,
    acpi_memory_nvs = EfiACPIMemoryNVS,
    memory_mapped_io = EfiMemoryMappedIO,
    memory_mapped_io_port = EfiMemoryMappedIOPortSpace,
    pal_code = EfiPalCode,
    persistent_memory = EfiPersistentMemory
};

class boot_services
{
public:
    using raw = _EFI_BOOT_SERVICES;
    boot_services(raw* raw_value);

    efi_result allocate_pages(allocate_type type, memory_type mem_type,
                              uintn pages, void** ptr);

    efi_result free_pages(void* address, uintn pages);

private:
    raw* m_raw;
};

} // namespace matrix_efi

#endif // MATRIX_EFI_BOOTLOADER_BOOT_SERVICES_H
