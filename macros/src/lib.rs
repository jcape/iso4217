//! Procedural Macros for Generating ISO 4217 Data

#![doc = include_str!("../README.md")]

mod xml;

use proc_macro::TokenStream;

/// Generate ISO 4217 data.
///
/// # Panics
///
/// If there was an error while parsing or generating data.
#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    xml::try_generate(input.into())
        .expect("Could not generate data")
        .into()
}
