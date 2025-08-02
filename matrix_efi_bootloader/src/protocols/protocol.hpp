#if !defined(MATRIX_EFI_PROTOCOL_H)
#define MATRIX_EFI_PROTOCOL_H

#include <efi.h>
#include "types.hpp"

namespace matrix_efi
{
    using efi_guid = EFI_GUID;

    template<typename T>
    concept efi_protocol = requires() {
        { T::guid() };
    };

    class test_protocol 
    {
        public:
        static efi_guid guid()
        {
            efi_guid res{};
            return res;
        };
    };

    template<efi_protocol T>
    void test()
    {
        test<test_protocol>();
    }

}

#endif // MATRIX_EFI_PROTOCOL_H
