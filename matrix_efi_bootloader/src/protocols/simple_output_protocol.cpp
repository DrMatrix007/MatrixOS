#include "simple_output_protocol.hpp"

#include "stdarg.h"

matrix_efi::simple_output_protocol::simple_output_protocol(raw* ptr)
    : m_raw(ptr)
{
}

void matrix_efi::simple_output_protocol::print(const CHAR16* fmt, ...)
{
    CHAR16 buffer[STR_LEN];
    for (int i = 0; i < STR_LEN; i++)
    {
        buffer[i] = 0;
    }
    CHAR16* buf_ptr = buffer;

    va_list args;
    va_start(args, fmt);

    for (const CHAR16* p = fmt; *p && (buf_ptr - buffer) < STR_LEN - 1; ++p)
    {
        if (*p == '%')
        {
            ++p;
            if (*p == 's')
            {
                CHAR16* str = va_arg(args, CHAR16*);
                while (*str && (buf_ptr - buffer) < STR_LEN - 1)
                    *buf_ptr++ = *str++;
            }
            else if (*p == 'd')
            {
                int val = va_arg(args, int);
                CHAR16 numbuf[20];
                int i = 0;
                bool neg = val < 0;
                if (neg)
                    val = -val;

                do
                {
                    numbuf[i++] = L'0' + (val % 10);
                    val /= 10;
                } while (val && i < 19);

                if (neg)
                    numbuf[i++] = L'-';

                // Reverse
                for (int j = i - 1; j >= 0; --j)
                    *buf_ptr++ = numbuf[j];
            }
            else if (*p == 'p')
            {
                void* ptr = va_arg(args, void*);
                uintptr_t val = (uintptr_t)ptr;

                *buf_ptr++ = L'0';
                *buf_ptr++ = L'x';

                CHAR16 numbuf[2 * sizeof(uintptr_t)];
                int i = 0;

                do
                {
                    int digit = val & 0xF;
                    if (digit < 10)
                        numbuf[i++] = L'0' + digit;
                    else
                        numbuf[i++] = L'a' + (digit - 10);
                    val >>= 4;
                } while (val && i < (int)(sizeof(numbuf) / sizeof(numbuf[0])));

                // reverse
                for (int j = i - 1; j >= 0; --j)
                    *buf_ptr++ = numbuf[j];
            }
            else
            {
                *buf_ptr++ = *p;
            }
        }
        else
        {
            *buf_ptr++ = *p;
        }
    }

    *buf_ptr = L'\0';
    va_end(args);

    output_string(buffer);
}

EFI_STATUS matrix_efi::simple_output_protocol::reset(bool extended)
{
    return m_raw->Reset(m_raw.get(), extended);
}

EFI_STATUS matrix_efi::simple_output_protocol::output_string(CHAR16* str)
{
    return m_raw->OutputString(m_raw.get(), str);
}

EFI_STATUS matrix_efi::simple_output_protocol::test_string(CHAR16* str)
{
    return m_raw->TestString(m_raw.get(), str);
}

EFI_STATUS matrix_efi::simple_output_protocol::query_mode(UINTN mode_number,
                                                          UINTN* columns,
                                                          UINTN* rows)
{
    return m_raw->QueryMode(m_raw.get(), mode_number, columns, rows);
}

EFI_STATUS matrix_efi::simple_output_protocol::set_mode(UINTN mode_number)
{
    return m_raw->SetMode(m_raw.get(), mode_number);
}

EFI_STATUS matrix_efi::simple_output_protocol::set_attribute(UINTN attribute)
{
    return m_raw->SetAttribute(m_raw.get(), attribute);
}

EFI_STATUS matrix_efi::simple_output_protocol::clear_screen()
{
    return m_raw->ClearScreen(m_raw.get());
}

EFI_STATUS matrix_efi::simple_output_protocol::set_cursor_position(UINTN column,
                                                                   UINTN row)
{
    return m_raw->SetCursorPosition(m_raw.get(), column, row);
}

EFI_STATUS matrix_efi::simple_output_protocol::enable_cursor(bool visible)
{
    return m_raw->EnableCursor(m_raw.get(), visible);
}

SIMPLE_TEXT_OUTPUT_MODE* matrix_efi::simple_output_protocol::mode()
{
    return m_raw->Mode;
}