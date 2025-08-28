#if !defined(MATRIX_EFI_PROTOCOL_H)
#define MATRIX_EFI_PROTOCOL_H

#include "type_traits/is_same.hpp"
#include "unique_handle.hpp"
#include "efi.h"
#include "efiapi.h"
#include "type_traits/is_same.hpp"

namespace matrix_efi
{
using efi_guid = EFI_GUID;

template <typename protocol> class protocol_handle;

template <typename t>
concept efi_protocol = requires() {
    { t::guid() } -> mst::same_as<efi_guid>;
} && requires() { typename t::raw; } && requires(protocol_handle<typename t::raw> p) {
    { t(mst::move(p)) } -> mst::same_as<t>;
} && requires(t prot) {
    { prot.get_raw() } -> mst::same_as<typename t::raw*>;
};

template <typename protocol_raw>
class protocol_handle : public unique_handle<protocol_raw>
{
public:
    protocol_handle(protocol_handle&&) = default;
    protocol_handle& operator=(protocol_handle&&) = default;
    protocol_handle(protocol_raw* raw, EFI_SYSTEM_TABLE* table, efi_guid guid)
        : unique_handle<protocol_raw>(raw), m_table(table), m_guid(guid)
    {
    }
    ~protocol_handle()
    {
        m_table->BootServices->CloseProtocol(m_table, &m_guid, nullptr,
                                             nullptr);
    }

private:
    EFI_SYSTEM_TABLE* m_table;
    efi_guid m_guid;
};

} // namespace matrix_efi

#endif // MATRIX_EFI_PROTOCOL_H
