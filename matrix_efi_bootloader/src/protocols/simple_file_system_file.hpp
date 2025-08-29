#ifndef MATRIX_EFI_BOOATLODER_PROTOCOLS_SIMPLE_FILE_SYSTEM_FILE_H
#define MATRIX_EFI_BOOATLODER_PROTOCOLS_SIMPLE_FILE_SYSTEM_FILE_H

#include "efi.h"
#include "efi_error.hpp"
#include "int_types.hpp"
#include "result.hpp"
#include "unique_handle.hpp"

namespace matrix_efi
{
class sub_files_range;
class simple_filesystem_file
{
public:
    using raw = _EFI_FILE_HANDLE;
    simple_filesystem_file(unique_handle<raw> raw);
    simple_filesystem_file(simple_filesystem_file&&) = default;
    simple_filesystem_file& operator=(simple_filesystem_file&&) = default;

    mst::result<simple_filesystem_file, efi_error> open(wchar_t* filename,
                                                        uint64 open_mode,
                                                        uint64 attributes);
    efi_result close();
    efi_result remove();
    efi_result read(void* buffer, uintn* buffer_size);
    efi_result write(void* buffer, uintn* buffer_size);
    mst::result<uint64, efi_error> get_position();
    efi_result set_position(uint64 position);
    efi_result flush();

    ~simple_filesystem_file();


private:
    unique_handle<raw> m_raw;
};

class sub_files_range
{
public:
    explicit sub_files_range(simple_filesystem_file* dir) : m_dir(dir)
    {
    }

    class iterator
    {
    public:
        iterator() = default;
        explicit iterator(simple_filesystem_file* dir) : m_dir(dir)
        {
            advance();
        }

        EFI_FILE_INFO operator*() const
        {
            return m_info;
        }
        iterator& operator++()
        {
            advance();
            return *this;
        }
        bool operator!=(const iterator& other) const
        {
            return m_dir != other.m_dir;
        }

    private:
        void advance()
        {
            if (!m_dir)
                return;

            char buffer[sizeof(EFI_FILE_INFO) / sizeof(char)];
            uintn size = sizeof(buffer);
            if (m_dir->read(buffer, &size).is_empty())
            {
                m_dir = nullptr;
                return;
            }

            auto* info = reinterpret_cast<EFI_FILE_INFO*>(buffer);

            m_info = *info; // copy the info out
        }

        simple_filesystem_file* m_dir{nullptr};
        EFI_FILE_INFO m_info{};
    };

    iterator begin()
    {
        return iterator(m_dir);
    }
    iterator end()
    {
        return iterator();
    }

private:
    simple_filesystem_file* m_dir;
};

// ------------------------------------------------------------
// Convenience wrapper
// ------------------------------------------------------------
inline sub_files_range sub_files(simple_filesystem_file& dir)
{
    return sub_files_range(&dir);
}

} // namespace matrix_efi

#endif // MATRIX_EFI_BOOATLODER_PROTOCOLS_SIMPLE_FILE_SYSTEM_FILE_H