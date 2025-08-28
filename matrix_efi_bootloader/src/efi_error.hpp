#ifndef MATRIX_EFI_BOOTLOADER_EFI_ERROR_H
#define MATRIX_EFI_BOOTLOADER_EFI_ERROR_H

#include "efi.h"
#include "optional.hpp"

namespace matrix_efi
{
using efi_status = EFI_STATUS;
constexpr efi_status efi_success = EFI_SUCCESS;
class efi_error
{
public:
    constexpr efi_error(efi_status status) : m_status(status)
    {
    }

    constexpr bool operator==(efi_status other) const noexcept
    {
        return other == m_status;
    }

    constexpr bool operator!=(efi_status other) const noexcept
    {
        return other != m_status;
    }

    constexpr efi_status raw() const noexcept
    {
        return m_status;
    }

private:
    efi_status m_status;
};

using efi_result = mst::optional<efi_error>;

constexpr efi_result make_efi_result(efi_status status)
{
    if (status != EFI_SUCCESS)
    {
        return ::mst::optional<efi_error>(efi_error(status));
    }
    return ::mst::nullopt;
}

} // namespace matrix_efi

#endif // MATRIX_EFI_BOOTLOADER_EFI_ERROR_H