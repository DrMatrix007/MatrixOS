export module mstd.variant;

import mstd.type_traits;
import mstd.semantics;
import mstd.custom_new;

namespace mstd
{
    using index = unsigned char;

    template <typename... args>
    struct variant_storage
    {
    };

    template <typename first_arg, typename... rest_args>
    struct variant_storage<first_arg, rest_args...>
    {
    public:
        constexpr variant_storage() : _rest()
        {
        }

        constexpr ~variant_storage()
        {
        }
        ;

    public:
        template <typename arg>
        constexpr index init(arg&& value);
        template <typename arg>
        constexpr index init(const arg& value);

        constexpr void de_init(index i);

        template <typename arg>
        constexpr arg& get();
        template <typename arg>
        constexpr const arg& get() const;

    public:
        union
        {
            first_arg _value;
            variant_storage<rest_args...> _rest;
        };
    };

    template <typename first_arg, typename... rest_args>
    template <typename arg>
    constexpr index variant_storage<first_arg, rest_args...>::init(arg&& value)
    {
        if constexpr (mstd::is_same<arg, first_arg>)
        {
            new(&_value) arg(value);
            return 0;
        }
        return 1 + _rest.init(mstd::move(value));
    }

    template <typename first_arg, typename... rest_args>
    template <typename arg>
    constexpr index variant_storage<first_arg, rest_args...>::init(const arg& value)
    {
        if constexpr (mstd::is_same<arg, first_arg>)
        {
            new(&_value) arg(value);
            return 0;
        }
        return 1 + _rest.init(mstd::move(value));
    }

    template <typename first_arg, typename... rest_args>
    constexpr void variant_storage<first_arg, rest_args...>::de_init(const index i)
    {
        if (i == 0)
        {
            _value.~first_arg();
        }
        else
        {
            _rest.de_init(i - 1);
        }
    }

    template <typename first_arg, typename... rest_args>
    template <typename arg>
    constexpr arg& variant_storage<first_arg, rest_args...>::get()
    {
        if constexpr (is_same<arg, first_arg>)
        {
            return _value;
        }
        return _rest.template get<arg>();
    }

    template <typename first_arg, typename... rest_args>
    template <typename arg>
    constexpr const arg& variant_storage<first_arg, rest_args...>::get() const
    {
        if constexpr (is_same<arg, first_arg>)
        {
            return _value;
        }
        return _rest.template get<arg>();
    }

    template <typename arg>
    struct variant_storage<arg>
    {
    public:
        constexpr variant_storage() : _dummy(0)
        {
        };

        constexpr ~variant_storage()
        {
        }

    public:
        template <typename other_arg>
        constexpr index init(other_arg&& value);
        template <typename other_arg>
        constexpr index init(const other_arg& value);

        constexpr void de_init(index i);

        template <typename other_arg>
        constexpr other_arg& get();
        template <typename other_arg>
        constexpr const other_arg& get() const;

    public:
        union
        {
            arg _value;
            char _dummy;
        };
    };

    template <typename arg>
    template <typename other_arg>
    constexpr index variant_storage<arg>::init(other_arg&& value)
    {
        static_assert(is_same<arg, other_arg>, "type is probably missing from variant");

        new(&_value) arg(mstd::move(value));

        return 0;
    }

    template <typename arg>
    template <typename other_arg>
    constexpr index variant_storage<arg>::init(const other_arg& value)
    {
        static_assert(is_same<arg, other_arg>, "type is probably missing from variant");

        new(&_value) arg(value);

        return 0;
    }

    template <typename arg>
    constexpr void variant_storage<arg>::de_init(const index i)
    {
        if (i == 0)
        {
            _value.~arg();
        }
    }

    template <typename arg>
    template <typename other_arg>
    constexpr other_arg& variant_storage<arg>::get()
    {
        static_assert(is_same<arg, other_arg>, "type is probably missing from variant");

        return _value;
    }

    template <typename arg>
    template <typename other_arg>
    constexpr const other_arg& variant_storage<arg>::get() const
    {
        return _value;
    }

    export template <typename... args>
    class variant;

    template <typename value, typename... args>
    class variant_iter
    {
    public:
        constexpr variant_iter(variant<args...>& variant);
        constexpr variant_iter(variant<args...>& variant, bool is_done);

    public:
        constexpr variant_iter& operator++();
        constexpr bool operator==(const variant_iter& other) const;

    public:
        constexpr value* operator->();
        constexpr const value* operator->() const;

        constexpr value& operator*();
        constexpr const value& operator*() const;

    private:
        variant<args...>& _variant;
        bool _is_done;
    };

    template <typename value, typename... args>
    class variant_iter_const
    {
    public:
        constexpr variant_iter_const(const variant<args...>& variant);
        constexpr variant_iter_const(const variant<args...>& variant, bool is_done);

    public:
        constexpr variant_iter_const& operator++();
        constexpr bool operator==(const variant_iter_const& other) const;

    public:
        constexpr const value* operator->() const;
        constexpr const value& operator*() const;

    private:
        const variant<args...>& _variant;
        bool _is_done;
    };


    export template <typename value, typename... args>
    class variant_getter
    {
    public:
        explicit constexpr variant_getter(variant<args...>& variant);

    public:
        constexpr variant_iter<value, args...> begin();
        constexpr variant_iter<value, args...> end();

    private:
        variant<args...>& _variant;
    };

    export template <typename value, typename... args>
    class variant_getter_const
    {
    public:
        explicit constexpr variant_getter_const(const variant<args...>& variant);

    public:
        constexpr variant_iter_const<value, args...> begin();
        constexpr variant_iter_const<value, args...> end();

    private:
        const variant<args...>& _variant;
    };


    export template <typename... args>
    class variant
    {
    public:
        template <typename arg>
        constexpr variant(arg&& value);
        constexpr ~variant();

    public:
        template <typename value>
        constexpr variant_getter<value, args...> get();

        template <typename value>
        constexpr variant_getter_const<value, args...> get() const;

    private:
        template <typename some_arg, typename... some_args>
        friend class variant_iter;
        template <typename some_arg, typename... some_args>
        friend class variant_iter_const;

        variant_storage<args...> _storage;
        index _index;
    };

    template <typename... args>
    template <typename value>
    constexpr variant_getter<value, args...> variant<args...>::get()
    {
        return variant_getter<value, args...>(*this);
    }

    template <typename ... args>
    template <typename value>
    constexpr variant_getter_const<value, args...> variant<args...>::get() const
    {
        return variant_getter_const<value, args...>(*this);
    }

    template <typename value, typename... args>
    constexpr variant_getter<value, args...>::variant_getter(variant<args...>& variant) :
        _variant(variant)
    {
    }

    template <typename value, typename... args>
    constexpr variant_iter<value, args...> variant_getter<value, args...>::begin()
    {
        return variant_iter<value, args...>(_variant);
    }

    template <typename value, typename... args>
    constexpr variant_iter<value, args...> variant_getter<value, args...>::end()
    {
        return variant_iter<value, args...>(_variant, true);
    }

    template <typename value, typename... args>
    constexpr variant_getter_const<value, args...>::variant_getter_const(const variant<args...>& variant) :
        _variant(variant)
    {
    }

    template <typename value, typename ... args>
    constexpr variant_iter_const<value, args...> variant_getter_const<value, args...>::begin()
    {
        return variant_iter_const<value, args...>(_variant);
    }

    template <typename value, typename ... args>
    constexpr variant_iter_const<value, args...> variant_getter_const<value, args...>::end()
    {
        return variant_iter_const<value, args...>(_variant, false);
    }

    template <typename arg, typename... args>
    constexpr variant_iter<arg, args...>::variant_iter(variant<args...>& variant) :
        variant_iter(variant, variant._index != index_of<arg, args...>)
    {
    }

    template <typename value, typename... args>
    constexpr variant_iter<value, args...>::variant_iter(variant<args...>& variant, bool is_done) :
        _variant(variant),
        _is_done(is_done)
    {
    }

    template <typename arg, typename... args>
    constexpr variant_iter<arg, args...>& variant_iter<arg, args...>::operator++()
    {
        _is_done = true;
        return *this;
    }

    template <typename arg, typename... args>
    constexpr bool variant_iter<arg, args...>::operator==(const variant_iter& other) const
    {
        return _is_done == other._is_done;
    }

    template <typename arg, typename... args>
    constexpr arg* variant_iter<arg, args...>::operator->()
    {
        return &_variant._storage.template get<arg>();
    }

    template <typename arg, typename... args>
    constexpr const arg* variant_iter<arg, args...>::operator->() const
    {
        return &_variant._storage.template get<arg>();
    }

    template <typename value, typename... args>
    constexpr value& variant_iter<value, args...>::operator*()
    {
        return *this->operator->();
    }

    template <typename value, typename... args>
    constexpr const value& variant_iter<value, args...>::operator*() const
    {
        return *this->operator->();
    }

    template <typename value, typename... args>
    constexpr variant_iter_const<value, args...>::variant_iter_const(const variant<args...>& variant) :
        variant_iter_const(variant, variant._index != index_of<value, args...>)

    {
    }

    template <typename value, typename ... args>
    constexpr variant_iter_const<value, args...>::variant_iter_const(const variant<args...>& variant, bool is_done):
    _variant(variant), _is_done(is_done)
    {
    }

    template <typename value, typename... args>
    constexpr variant_iter_const<value, args...>& variant_iter_const<value, args...>::operator++()
    {
        _is_done = true;
        return *this;
    }

    template <typename value, typename... args>
    constexpr bool variant_iter_const<value, args...>::operator==(const variant_iter_const& other) const
    {
        return _is_done == other._is_done;
    }

    template <typename value, typename ... args>
    constexpr const value* variant_iter_const<value, args...>::operator->() const
    {
        return &_variant._storage.template get<value>();
    }

    template <typename value, typename ... args>
    constexpr const value& variant_iter_const<value, args...>::operator*() const
    {
        return *this->operator->();
    }

    template <typename... args>
    template <typename arg>
    constexpr variant<args...>::variant(arg&& value) :
        _storage(),
        _index()
    {
        _index = _storage.init(value);
    }

    template <typename... args>
    constexpr variant<args...>::~variant()
    {
        _storage.de_init(_index);
    }
}
