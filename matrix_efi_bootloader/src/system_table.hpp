// #if !defined(MATRIX_EFI_SYSTEM_TABLE_H)
// #define MATRIX_EFI_SYSTEM_TABLE_H

// #include <efi.h>
// #include <efilib.h>
// #include <efiapi.h>

// #include "protocols/protocol.hpp"

// namespace matrix_efi
// {
//     using raw_system_table = EFI_SYSTEM_TABLE;
//     class system_table
//     {
//     public:
//         system_table(raw_system_table* ptr);

//         template<efi_protocol protocol>
//         protocol get_protocol();
//     private:
//         raw_system_table *m_raw;
    
    
//     };

//     template <efi_protocol protocol>
//     inline protocol system_table::get_protocol()
//     {
//         return protocol();
//     }

// }

// #endif // MATRIX_EFI_SYSTEM_TABLE_H
