#[derive(Debug, Default, Clone, Copy)]
pub struct ValueMismatch;

#[macro_export]
macro_rules! impl_try_from_enum_values {
    ($from_type:ty, $enum_name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        impl TryFrom<$from_type> for $enum_name {
            type Error = ValueMismatch;

            fn try_from(value: $from_type) -> core::result::Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok($enum_name::$variant),
                    )*
                    _ => Err(ValueMismatch),
                }
            }
        }
    };
}
