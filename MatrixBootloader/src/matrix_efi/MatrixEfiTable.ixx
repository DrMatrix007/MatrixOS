module;

#include <efi.h>
#include <efilib.h>

export module MatrixEfiTable;

namespace mefi
{
    export class MatrixEfiTable
    {
    public:
        MatrixEfiTable() = default;

    private:
        EFI_SYSTEM_TABLE* _table;
    };
}