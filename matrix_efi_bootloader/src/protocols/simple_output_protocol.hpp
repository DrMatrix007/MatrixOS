#ifndef MATRIX_EFI_SIMPLE_OUTPUT_PROTOCOL_H
#define MATRIX_EFI_SIMPLE_OUTPUT_PROTOCOL_H

#include "efi.h"
#include "efilib.h"
#include <efipciio.h>

#define STR_LEN 500

namespace matrix_efi
{
using raw_simple_output_protocol = EFI_SIMPLE_TEXT_OUT_PROTOCOL;

class simple_output_protocol
{
public:
    simple_output_protocol(raw_simple_output_protocol* ptr);

    EFI_STATUS reset(bool extended);
    EFI_STATUS output_string(CHAR16* str);
    EFI_STATUS test_string(CHAR16* str);

    EFI_STATUS query_mode(UINTN mode_number, UINTN* columns, UINTN* rows);
    EFI_STATUS set_mode(UINTN mode_number);
    EFI_STATUS set_attribute(UINTN attribute);

    EFI_STATUS clear_screen();
    EFI_STATUS set_cursor_position(UINTN column, UINTN row);
    EFI_STATUS enable_cursor(bool visible);

    SIMPLE_TEXT_OUTPUT_MODE* mode();

    void print(const CHAR16* fmt, ...);

private:
    raw_simple_output_protocol* m_raw;
};

} // namespace matrix_efi

#endif // MATRIX_EFI_SIMPLE_OUTPUT_PROTOCOL_H
