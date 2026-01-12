//! Static ISO 4217 Data

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "serde")]
mod _serde;

use core::str::FromStr;
use iso3166_static::{Alpha2, Alpha3, Numeric};

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

    /// Whether this error is of the `InvalidCharset` variant.
    #[must_use]
    pub const fn is_no_universal_currency(&self) -> bool {
        matches!(self, Self::NoUniversalCurrency)
    }
}

impl AsRef<str> for Currency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
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

impl TryFrom<Numeric> for Currency {
    type Error = Error;

    fn try_from(value: Numeric) -> Result<Self, Self::Error> {
        Self::from_numeric_country(value).ok_or(Error::NoUniversalCurrency)
    }
}

impl TryFrom<Alpha2> for Currency {
    type Error = Error;

    fn try_from(value: Alpha2) -> Result<Self, Self::Error> {
        Self::from_alpha2_country(value).ok_or(Error::NoUniversalCurrency)
    }
}

impl TryFrom<Alpha3> for Currency {
    type Error = Error;

    fn try_from(value: Alpha3) -> Result<Self, Self::Error> {
        Self::from_alpha3_country(value).ok_or(Error::NoUniversalCurrency)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use iso3166_static::{Alpha2, Alpha3, Numeric};

    #[test]
    fn for_country() {
        const NO_CURRENCY: &[Numeric] = &[
            Numeric::Antarctica,
            Numeric::SouthGeorgiaAndTheSouthSandwichIslands,
            Numeric::Palestine,
        ];

        for i in 1..=899 {
            // if the numeric code is valid, and it's not user-assigned.
            if let Ok(numeric) = Numeric::from_u16(i)
                && let Ok(alpha2) = Alpha2::from_numeric(numeric)
                && let Ok(alpha3) = Alpha3::from_numeric(numeric)
            {
                let numeric_currency = Currency::try_from(numeric);
                let alpha2_currency = Currency::try_from(alpha2);
                let alpha3_currency = Currency::try_from(alpha3);

                if NO_CURRENCY.contains(&numeric) {
                    assert_eq!(
                        Error::NoUniversalCurrency,
                        numeric_currency.expect_err("no currency")
                    );
                    assert_eq!(
                        Error::NoUniversalCurrency,
                        alpha2_currency.expect_err("no currency")
                    );
                    assert_eq!(
                        Error::NoUniversalCurrency,
                        alpha3_currency.expect_err("no currency")
                    );
                } else {
                    assert!(numeric_currency.is_ok());
                    assert!(alpha2_currency.is_ok());
                    assert!(alpha3_currency.is_ok());
                }
            }
        }
    }
}
