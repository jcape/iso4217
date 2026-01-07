//! Static ISO 4217 Data

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "serde")]
mod _serde;

use core::str::FromStr;

iso4217_macros::generate!(xml = "list-one.xml");

impl Error {
    /// Whether this error is of the `InvalidCode` variant.
    #[must_use]
    pub const fn is_invalid_code(&self) -> bool {
        matches!(self, Self::InvalidCode)
    }

    /// Whether this error is of the `InvalidLength` variant.
    #[must_use]
    pub const fn is_invalid_length(&self) -> bool {
        matches!(self, Self::InvalidLength)
    }

    /// Whether this error is of the `InvalidCharset` variant.
    #[must_use]
    pub const fn is_invalid_charset(&self) -> bool {
        matches!(self, Self::InvalidCharset)
    }
}

impl TryFrom<u16> for Currency {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl TryFrom<&str> for Currency {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_slice(value)
    }
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl AsRef<str> for Currency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
