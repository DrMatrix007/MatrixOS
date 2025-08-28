#ifndef MATRIX_EFI_BOOTLOADER_EFI_UNIQUE_HANDLE_H
#define MATRIX_EFI_BOOTLOADER_EFI_UNIQUE_HANDLE_H

#include "mem_utils.hpp"
namespace matrix_efi
{

template <typename raw> class unique_handle
{
public:
    unique_handle(raw* raw_val = nullptr) noexcept : m_raw(raw_val)
    {
    }

    unique_handle(const unique_handle&) = delete;
    unique_handle& operator=(const unique_handle&) = delete;

    unique_handle(unique_handle&& other) noexcept : m_raw(other.m_raw)
    {
        other.m_raw = nullptr;
    }

    unique_handle& operator=(unique_handle&& other) noexcept
    {
        if (this != &other)
        {
            mst::swap(m_raw, other.m_raw);
        }
        return *this;
    }

    raw* operator->() noexcept
    {
        return m_raw;
    }
    const raw* operator->() const noexcept
    {
        return m_raw;
    }

    raw& operator*() noexcept
    {
        return *m_raw;
    }
    const raw& operator*() const noexcept
    {
        return *m_raw;
    }

    raw* get() noexcept
    {
        return m_raw;
    }
    const raw* get() const noexcept
    {
        return m_raw;
    }

    explicit operator bool() const noexcept
    {
        return m_raw != nullptr;
    }

    raw* release() noexcept
    {
        raw* tmp = m_raw;
        m_raw = nullptr;
        return tmp;
    }

protected:
    raw* m_raw;
};
} // namespace matrix_efi

#endif // MATRIX_EFI_BOOTLOADER_EFI_UNIQUE_HANDLE_H