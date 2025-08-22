#include "type_traits/is_same.hpp"
#if !defined(MATRIX_EFI_PROTOCOL_H)
#define MATRIX_EFI_PROTOCOL_H

#include "type_traits/is_same.hpp"
#include <efi.h>

namespace matrix_efi
{
using efi_guid = EFI_GUID;

template <typename t>
concept efi_protocol = requires() {
    { t::guid() } -> mst::same_as<efi_guid>;
} && requires() {
    typename t::raw;
} && requires(t::raw* p) {
    { t(p) } -> mst::same_as<t>;
};

} // namespace matrix_efi

#endif // MATRIX_EFI_PROTOCOL_H
