#include "boot_services.hpp"
#include "efi_error.hpp"
#include "efidef.h"

namespace matrix_efi
{
boot_services::boot_services(raw* raw_value) : m_raw(raw_value)
{
}

efi_result boot_services::allocate_pages(
    allocate_type type, memory_type mem_type, uintn pages, void** ptr)
{
    uint64 ptr_tmp = reinterpret_cast<decltype(ptr_tmp)>(*ptr);

    efi_status status = m_raw->AllocatePages(
        static_cast<EFI_ALLOCATE_TYPE>(type),
        static_cast<EFI_MEMORY_TYPE>(mem_type), pages, &ptr_tmp);

    if (status != efi_success)
    {
        return (efi_error)status;
    }
    *ptr = reinterpret_cast<decltype(*ptr)>(ptr_tmp);
    return {};
}

efi_result boot_services::free_pages(void* address, uintn pages)
{
    efi_status res = m_raw->FreePages(reinterpret_cast<uint64>(address), pages);
    if (res != efi_success)
    {
        return (efi_error)res;
    }
    return {};
}

void boot_services::set_mem(void* buff, uintn size, uint8 val)
{
    m_raw->SetMem(buff, size, val);
}

} // namespace matrix_efi