# Static ISO 4217 Currency Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate provides a `no-std` and `no-std::no-alloc` capable enumeration of ISO 4217 Currency-code data, vis-a-vis the [`Currency`](crate::Currency) Rust enum. It requires Rust 1.88.0 or later, and MSRV updates are not considered breaking changes.

## Features

- `default`: Enables the `serde` feature.
- `serde`: Enables serialization/deserialization using `serde`.
- `alloc`: Enables the use of allocated types (this should be enabled if `serde` is enabled).
- `zerocopy`: Enables the derivation of [`zerocopy`](https://docs.rs/zerocopy) traits (specifically, [`TryFromBytes`](zerocopy::TryFromBytes) and [`IntoBytes`](zerocopy::IntoBytes)) on the [`Currency`](crate::Currency) enum.

## Examples

The enumeration can be created from the three-character currency code:

```rust
use iso4217_static::Currency;

const CURRENCY: &str = "USD";

let actual = Currency::try_from(CURRENCY).expect("valid code");
assert_eq!(Currency::UsDollar, actual);
assert_eq!(CURRENCY, actual.as_ref());
```

Numeric codes are also supported:

```rust
use iso4217_static::Currency;

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

let actual = Currency::try_from(Alpha2::UnitedStatesOfAmerica).expect("country");
assert_eq!(Currency::UsDollar, actual);
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso4217-static.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso4217-static/0.2.1
[docs-image]: https://img.shields.io/docsrs/iso4217-static?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso4217-static/0.2.1/iso4217_static
[msrv-image]: https://img.shields.io/crates/msrv/iso4217-static/0.2.1?style=for-the-badge
