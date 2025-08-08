#if !defined(STANDARD_MATRIX_VARIANT_H)
#define STANDARD_MATRIX_VARIANT_H

#include "int_types.hpp"
#include "math.hpp"
#include "move.hpp"
#include "type_traits.hpp"
#include "mem_utils.hpp"
#include <new>
#include <bit>

namespace mst
{

    template <uint64 size, uint64 align>
    struct aligned_storage
    {

    public:
        constexpr aligned_storage() = default;

        constexpr void *as_ptr();

        template <smaller_than<size, align> T>
        constexpr T *as_ptr();

        template <smaller_than<size, align> T>
        constexpr const T *as_ptr() const;

        alignas(align) unsigned char m_data[size];
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    class variant
    {
    public:
        template <typename type>
            requires(find_type_index_v<type, types...> >= 0)
        constexpr variant(type &&arg);

    private:
        static constexpr uint64 size = max(sizeof(types)...);
        static constexpr uint64 align = max(alignof(types)...);

        constexpr void destruct_value();
        template <uint64 index>
        constexpr void destruct_value_impl();

        aligned_storage<size, align> m_storage;
        uint64 m_index;
    };

    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <typename type>
        requires(find_type_index_v<type, types...> >= 0)
    constexpr variant<types...>::variant(type &&arg)
    {
        m_index = find_type_index_v<type, types...>;
        type *ptr = (type *)m_storage.as_ptr();
        new (ptr) type(move(arg));
    }
    template <uint64 size, uint64 align>
    constexpr inline void *aligned_storage<size, align>::as_ptr()
    {
        void *ptr = (void *)&m_data[0];
        unsigned long long val;

        void *ptr1 = &ptr;
        void *ptr2 = &val;

        unsigned long long *ptr3 = (unsigned long long*)&ptr;
        unsigned long long *ptr4 = (unsigned long long*)&val;

        return (void *)ptr;
    }

    template <uint64 size, uint64 align>
    template <smaller_than<size, align> T>
    constexpr inline T *aligned_storage<size, align>::as_ptr()
    {
        // void* ptr = as_ptr();
        // void** ptr1 = &ptr;
        // T*
        return nullptr;
        // return reinterpret_cast<T*>(&m_data[0]);;
    }

    template <uint64 size, uint64 align>
    template <smaller_than<size, align> T>
    constexpr inline const T *aligned_storage<size, align>::as_ptr() const
    {
        return static_cast<const T *>(static_cast<const void *>(&m_data[0]));
    }
    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    constexpr inline void variant<types...>::destruct_value()
    {
        destruct_value_impl<0>();
    }
    template <typename... types>
        requires(is_unique_tuple_v<types...>)
    template <uint64 index>
    constexpr inline void variant<types...>::destruct_value_impl()
    {
        if constexpr (index < sizeof...(types))
        {
            if (m_index == index)
            {
                m_storage.template as_ptr<type_index_t<index, types...>>();
            }
            destruct_value_impl<index + 1>();
        }
    }
}

#endif // STANDARD_MATRIX_VARIANT_H
