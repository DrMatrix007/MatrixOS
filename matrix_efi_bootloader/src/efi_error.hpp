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

    constexpr const wchar_t* as_string()
    {
        switch (m_status)
        {
            // Success
            case EFI_SUCCESS: return L"EFI_SUCCESS";

            // Warnings
            case EFI_WARN_UNKNOWN_GLYPH: return L"EFI_WARN_UNKNOWN_GLYPH";
            case EFI_WARN_DELETE_FAILURE: return L"EFI_WARN_DELETE_FAILURE";
            case EFI_WARN_WRITE_FAILURE: return L"EFI_WARN_WRITE_FAILURE";
            case EFI_WARN_BUFFER_TOO_SMALL: return L"EFI_WARN_BUFFER_TOO_SMALL";
            case EFI_WARN_STALE_DATA: return L"EFI_WARN_STALE_DATA";
            case EFI_WARN_FILE_SYSTEM: return L"EFI_WARN_FILE_SYSTEM";
            case EFI_WARN_RESET_REQUIRED: return L"EFI_WARN_RESET_REQUIRED";

            // Errors
            case EFI_LOAD_ERROR: return L"EFI_LOAD_ERROR";
            case EFI_INVALID_PARAMETER: return L"EFI_INVALID_PARAMETER";
            case EFI_UNSUPPORTED: return L"EFI_UNSUPPORTED";
            case EFI_BAD_BUFFER_SIZE: return L"EFI_BAD_BUFFER_SIZE";
            case EFI_BUFFER_TOO_SMALL: return L"EFI_BUFFER_TOO_SMALL";
            case EFI_NOT_READY: return L"EFI_NOT_READY";
            case EFI_DEVICE_ERROR: return L"EFI_DEVICE_ERROR";
            case EFI_WRITE_PROTECTED: return L"EFI_WRITE_PROTECTED";
            case EFI_OUT_OF_RESOURCES: return L"EFI_OUT_OF_RESOURCES";
            case EFI_VOLUME_CORRUPTED: return L"EFI_VOLUME_CORRUPTED";
            case EFI_VOLUME_FULL: return L"EFI_VOLUME_FULL";
            case EFI_NO_MEDIA: return L"EFI_NO_MEDIA";
            case EFI_MEDIA_CHANGED: return L"EFI_MEDIA_CHANGED";
            case EFI_NOT_FOUND: return L"EFI_NOT_FOUND";
            case EFI_ACCESS_DENIED: return L"EFI_ACCESS_DENIED";
            case EFI_NO_RESPONSE: return L"EFI_NO_RESPONSE";
            case EFI_NO_MAPPING: return L"EFI_NO_MAPPING";
            case EFI_TIMEOUT: return L"EFI_TIMEOUT";
            case EFI_NOT_STARTED: return L"EFI_NOT_STARTED";
            case EFI_ALREADY_STARTED: return L"EFI_ALREADY_STARTED";
            case EFI_ABORTED: return L"EFI_ABORTED";
            case EFI_ICMP_ERROR: return L"EFI_ICMP_ERROR";
            case EFI_TFTP_ERROR: return L"EFI_TFTP_ERROR";
            case EFI_PROTOCOL_ERROR: return L"EFI_PROTOCOL_ERROR";
            case EFI_INCOMPATIBLE_VERSION: return L"EFI_INCOMPATIBLE_VERSION";
            case EFI_SECURITY_VIOLATION: return L"EFI_SECURITY_VIOLATION";
            case EFI_CRC_ERROR: return L"EFI_CRC_ERROR";
            case EFI_END_OF_MEDIA: return L"EFI_END_OF_MEDIA";
            case EFI_END_OF_FILE: return L"EFI_END_OF_FILE";
            case EFI_INVALID_LANGUAGE: return L"EFI_INVALID_LANGUAGE";
            case EFI_COMPROMISED_DATA: return L"EFI_COMPROMISED_DATA";
            case EFI_IP_ADDRESS_CONFLICT: return L"EFI_IP_ADDRESS_CONFLICT";
            case EFI_HTTP_ERROR: return L"EFI_HTTP_ERROR";

            default:
                return L"UNKNOWN";
        }
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