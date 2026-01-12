# ISO 4217 XML Parser

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate provides the data structures necessasry to parse ISO 4217 XML data with [`quick-xml`](https://docs.rs/quick-xml).

As an end-user, this probably isn't the crate you're looking for, you probably want the fully enumerated data found in the [`iso4217-static`](https://crates.io/crates/iso4217-static) crate, which uses this crate (via a proc-macro) to generate enumerations and static data.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso4217-parser.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso4217-parser/0.2.1
[docs-image]: https://img.shields.io/docsrs/iso4217-parser?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso4217-parser/0.2.1/iso4217_parser
[msrv-image]: https://img.shields.io/crates/msrv/iso4217-parser/0.2.1?style=for-the-badge
