//! Serde support for ISO 4217.

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::{Currency, Error};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt::{Formatter, Result as FmtResult};
use serde::de::{Error as DeError, Unexpected, Visitor};

/// Serialize/Deserialize an ISO 4217 currency code as a string.
///
/// ```
/// use iso4217_static::Currency;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct MyStruct {
///     #[serde(with = "iso4217_static::serde::str")]
///     currency: Currency,
/// }
///
/// const EXPECTED: &str = r#"{"currency":"USD"}"#;
///
/// let input = MyStruct { currency: Currency::UsDollar };
/// let actual = serde_json::to_string(&input).expect("ser");
///
/// assert_eq!(EXPECTED, &actual);
///
/// let output = serde_json::from_str(&actual).expect("de");
///
/// assert_eq!(input, output);
/// ```
pub mod str {
    use crate::{Currency, serde::CurrencyVisitor};
    use serde::{Deserializer, Serialize, Serializer};

    /// Deserialize a given value into a `Currency`.
    ///
    /// # Errors
    ///
    /// - Returns an error if there is a problem deserializing the value.
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Currency, D::Error> {
        deserializer.deserialize_any(CurrencyVisitor)
    }

    /// Serialize a `Currency` into a string.
    ///
    /// # Errors
    ///
    /// - Returns an error if there is a problem serializing the value.
    pub fn serialize<S: Serializer>(value: &Currency, serializer: S) -> Result<S::Ok, S::Error> {
        value.as_str().serialize(serializer)
    }
}

/// Serialize/Deserialize an ISO 4217 currency code as a numeric value.
///
/// # Examples
///
/// ```
/// use iso4217_static::Currency;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct MyStruct {
///     #[serde(with = "iso4217_static::serde::u16")]
///     currency: Currency,
/// }
///
/// const EXPECTED: &str = r#"{"currency":840}"#;
///
/// let input = MyStruct { currency: Currency::UsDollar };
/// let actual = serde_json::to_string(&input).expect("ser");
///
/// assert_eq!(EXPECTED, &actual);
///
/// let output = serde_json::from_str(&actual).expect("de");
///
/// assert_eq!(input, output);
/// ```
pub mod u16 {
    use crate::{Currency, serde::CurrencyVisitor};
    use serde::{Deserializer, Serialize, Serializer};

    /// Deserialize a given value into a `Currency`.
    ///
    /// # Errors
    ///
    /// - Returns an error if there is a problem deserializing the value.
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Currency, D::Error> {
        deserializer.deserialize_any(CurrencyVisitor)
    }

    /// Serialize a `Currency` into a u16 value.
    ///
    /// # Errors
    ///
    /// - Returns an error if there is a problem serializing the value.
    pub fn serialize<S: Serializer>(value: &Currency, serializer: S) -> Result<S::Ok, S::Error> {
        (*value as u16).serialize(serializer)
    }
}

/// A visitor for deserializing a currency value.
struct CurrencyVisitor;

impl<'de> Visitor<'de> for CurrencyVisitor {
    type Value = Currency;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a valid currency value")
    }

    fn visit_i16<E: DeError>(self, v: i16) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Signed(i64::from(v)), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_i32<E: DeError>(self, v: i32) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Signed(i64::from(v)), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Signed(v), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_i128<E: DeError>(self, v: i128) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Bytes(&v.to_le_bytes()), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_u16<E: DeError>(self, v: u16) -> Result<Self::Value, E> {
        Currency::from_u16(v)
            .map_err(|_error| E::invalid_value(Unexpected::Unsigned(u64::from(v)), &self))
    }

    fn visit_u32<E: DeError>(self, v: u32) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Unsigned(u64::from(v)), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Unsigned(v), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_u128<E: DeError>(self, v: u128) -> Result<Self::Value, E> {
        u16::try_from(v)
            .map_err(|_error| E::invalid_value(Unexpected::Bytes(&v.to_le_bytes()), &self))
            .and_then(|value| self.visit_u16(value))
    }

    fn visit_f32<E: DeError>(self, v: f32) -> Result<Self::Value, E> {
        self.visit_f64(f64::from(v))
    }

    fn visit_f64<E: DeError>(self, v: f64) -> Result<Self::Value, E> {
        Err(E::invalid_type(Unexpected::Float(v), &self))
    }

    fn visit_char<E: DeError>(self, v: char) -> Result<Self::Value, E> {
        self.visit_str(v.encode_utf8(&mut [0u8; 4]))
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        Currency::from_str_slice(v).map_err(|error| match error {
            Error::InvalidCode | Error::InvalidCharset | Error::NoUniversalCurrency => {
                E::invalid_value(Unexpected::Str(v), &self)
            }
            Error::InvalidLength => E::invalid_length(3, &self),
        })
    }

    fn visit_borrowed_str<E: DeError>(self, v: &'de str) -> Result<Self::Value, E> {
        self.visit_str(v)
    }

    #[cfg(feature = "alloc")]
    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(&v)
    }

    fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
        if v.len() >= 3 {
            str::from_utf8(&v[..3])
                .map_err(|_e| E::invalid_value(Unexpected::Bytes(v), &self))
                .and_then(|value| {
                    Currency::from_str_slice(value).map_err(|error| match error {
                        Error::InvalidCode | Error::InvalidCharset | Error::NoUniversalCurrency => {
                            E::invalid_value(Unexpected::Bytes(v), &self)
                        }
                        Error::InvalidLength => E::invalid_length(3, &self),
                    })
                })
        } else if v.len() == 2 {
            let mut bytes = [0u8; 2];
            bytes.copy_from_slice(&v[..2]);

            Currency::from_u16(u16::from_le_bytes(bytes)).map_err(|error| match error {
                Error::InvalidCode | Error::InvalidCharset | Error::NoUniversalCurrency => {
                    E::invalid_value(Unexpected::Bytes(v), &self)
                }
                Error::InvalidLength => E::invalid_length(3, &self),
            })
        } else {
            Err(E::invalid_length(3, &self))
        }
    }

    fn visit_borrowed_bytes<E: DeError>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        self.visit_bytes(v)
    }

    #[cfg(feature = "alloc")]
    fn visit_byte_buf<E: DeError>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        self.visit_bytes(&v)
    }
}
