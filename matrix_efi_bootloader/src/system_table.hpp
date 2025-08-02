#if !defined(MATRIX_EFI_SYSTEM_TABLE_H)
#define MATRIX_EFI_SYSTEM_TABLE_H

#include <efi.h>
#include <efilib.h>
#include <efiapi.h>

namespace matrix_efi
{
    using raw_system_table = EFI_SYSTEM_TABLE;
    class system_table
    {
    public:
        system_table(raw_system_table* ptr);
    private:
        raw_system_table *m_table;
    
    
    };

}

#endif // MATRIX_EFI_SYSTEM_TABLE_H
