#include "simple_output_protocol.hpp"
matrix_efi::simple_output_protocol::simple_output_protocol(raw_simple_output_protocol *ptr) : m_raw(ptr)
{
}

EFI_STATUS matrix_efi::simple_output_protocol::reset(bool extended)
{
    return m_raw->Reset(m_raw, extended);
}

EFI_STATUS matrix_efi::simple_output_protocol::output_string(CHAR16 *str)
{
    return m_raw->OutputString(m_raw, str);
}

EFI_STATUS matrix_efi::simple_output_protocol::test_string(CHAR16 *str)
{
    return m_raw->TestString(m_raw, str);
}

EFI_STATUS matrix_efi::simple_output_protocol::query_mode(UINTN mode_number, UINTN *columns, UINTN *rows)
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

EFI_STATUS matrix_efi::simple_output_protocol::set_cursor_position(UINTN column, UINTN row)
{
    return m_raw->SetCursorPosition(m_raw, column, row);
}

EFI_STATUS matrix_efi::simple_output_protocol::enable_cursor(bool visible)
{
    return m_raw->EnableCursor(m_raw, visible);
}

SIMPLE_TEXT_OUTPUT_MODE *matrix_efi::simple_output_protocol::mode()
{
    return m_raw->Mode;
}