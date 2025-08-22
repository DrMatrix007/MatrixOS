#include "simple_output_protocol.hpp"
matrix_efi::simple_output_protocol::simple_output_protocol(
    raw_simple_output_protocol* ptr)
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
    return m_raw->Reset(m_raw, extended);
}

EFI_STATUS matrix_efi::simple_output_protocol::output_string(CHAR16* str)
{
    return m_raw->OutputString(m_raw, str);
}

EFI_STATUS matrix_efi::simple_output_protocol::test_string(CHAR16* str)
{
    return m_raw->TestString(m_raw, str);
}

EFI_STATUS matrix_efi::simple_output_protocol::query_mode(UINTN mode_number,
                                                          UINTN* columns,
                                                          UINTN* rows)
{
    return m_raw->QueryMode(m_raw, mode_number, columns, rows);
}

EFI_STATUS matrix_efi::simple_output_protocol::set_mode(UINTN mode_number)
{
    return m_raw->SetMode(m_raw, mode_number);
}

EFI_STATUS matrix_efi::simple_output_protocol::set_attribute(UINTN attribute)
{
    return m_raw->SetAttribute(m_raw, attribute);
}

EFI_STATUS matrix_efi::simple_output_protocol::clear_screen()
{
    return m_raw->ClearScreen(m_raw);
}

EFI_STATUS matrix_efi::simple_output_protocol::set_cursor_position(UINTN column,
                                                                   UINTN row)
{
    return m_raw->SetCursorPosition(m_raw, column, row);
}

EFI_STATUS matrix_efi::simple_output_protocol::enable_cursor(bool visible)
{
    return m_raw->EnableCursor(m_raw, visible);
}

SIMPLE_TEXT_OUTPUT_MODE* matrix_efi::simple_output_protocol::mode()
{
    return m_raw->Mode;
}