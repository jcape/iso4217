# Procedural Macros for Generating ISO 4217 Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate provides the procedural macro necessary to generate data parsed from [`iso4217-parser`](https://crates.io/iso4217-parser).

As an end-user, this probably isn't the crate you're looking for, you probably want the fully enumerated data found in the [`iso4217-static`](https://crates.io/crates/iso4217-static) crate, which uses this crate to generate enumerations and static data.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso4217-macros.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso4217-macros/0.2.0
[docs-image]: https://img.shields.io/docsrs/iso4217-macros?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso4217-macros/0.2.0/iso4217_macros
[msrv-image]: https://img.shields.io/crates/msrv/iso4217-macros/0.2.0?style=for-the-badge
