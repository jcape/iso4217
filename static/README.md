# ISO 4217 Static Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->![License][license-image]

This crate provides generated ISO 4217 data. The primary point is the [`Currency`] enum, which contains enumrated values for the distributed currencies.

## Examples

The enumeration can be created from the three-character currency code:

```rust
use iso4217_static::Currency;
use std::str::FromStr;

const CURRENCY: &str = "USD";

let actual = Currency::from_str(CURRENCY).expect("valid code");
assert_eq!(Currency::UsDollar, actual);
assert_eq!(CURRENCY, actual.as_ref());
```

Numeric codes are also supported:

```rust
use iso4217_static::Currency;
use std::str::FromStr;

const CURRENCY: u16 = 840;

let actual = Currency::try_from(CURRENCY).expect("valid code");
assert_eq!(Currency::UsDollar, actual);
assert_eq!(CURRENCY, actual as u16);
```

There are also methods to get the universal currency for a given country (if
the country has one):

```rust
use iso3166_static::Alpha2;
use iso4217_static::Currency;
use std::str::FromStr;

let actual = Currency::try_from(Alpha2::UnitedStatesOfAmerica).expect("country");
assert_eq!(Currency::UsDollar, actual);
```

[//]: # (badges)

[license-image]: https://img.shields.io/github/license/jcape/iso4217?style=for-the-badge
[crate-image]: https://img.shields.io/crates/v/iso4217-static.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso4217-static/0.1.0
[docs-image]: https://img.shields.io/docsrs/iso4217-static?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso4217-static/0.1.0/iso4217_static
[deps-image]: https://deps.rs/crate/iso4217-static/0.1.0/status.svg?style=for-the-badge
[deps-link]: https://deps.rs/crate/iso4217-static/0.1.0
