//! XML Generation

mod config;
mod recordset;

use self::config::Config;
use crate::xml::recordset::EntrySet;
use iso4217_parser::{CurrencyDoc, CurrencyEntry};
use proc_macro2::TokenStream;
use quick_xml::de;
use std::{env, fs::File, io::BufReader};
use syn::{Error, Meta, Result, Token, parse::Parser, punctuated::Punctuated};

fn build_error() -> TokenStream {
    quote::quote! {
        /// ISO 4217 Currency Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
        pub enum Error {
            /// The code given is not a correct string or numeric code.
            InvalidCode,
            /// The code string given is the wrong length to be a currency code.
            InvalidLength,
            /// The code string given contains non-ASCII characters.
            InvalidCharset,
        }
    }
}

/// Generate the currency code enum.
fn build_enum(entryset: &EntrySet) -> TokenStream {
    let doc = entryset.doc();
    let id = entryset.ident();
    let num = entryset.number();

    quote::quote! {
        /// ISO 4217 Currency Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
        #[repr(u16)]
        pub enum Currency {
            #(
                #[doc = #doc]
                #id = #num,
            )*
        }
    }
}

fn build_impl(entryset: &EntrySet) -> TokenStream {
    let currency = entryset.currency();
    let bytes = entryset.bytes();
    let id = entryset.ident();
    let number = entryset.number();
    let name = entryset.name();
    let is_fund = entryset.is_fund();
    let minor_unit = entryset.minor_unit();

    quote::quote! {
        impl Currency {
            /// Try to derive a currency from the given numeric code.
            pub const fn from_u16(value: u16) -> Result<Self, Error> {
                match value {
                    #(
                        #number => Ok(Self::#id),
                    )*

                    _ => Err(Error::InvalidCode),
                }
            }

            /// Try to derive a currency from the given ASCII string slice.
            pub const fn from_str_slice(value: &str) -> Result<Self, Error> {
                if value.len() != 3 {
                    return Err(Error::InvalidLength);
                }

                if !value.is_ascii() {
                    return Err(Error::InvalidCharset);
                }

                match value.as_bytes() {
                    #(
                        #bytes => Ok(Self::#id),
                    )*

                    _ => Err(Error::InvalidCode),
                }
            }

            /// Get the string code for this currency value.
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(
                        Self::#id => #currency,
                    )*
                }
            }

            /// Get the string code for this currency value.
            pub const fn name(&self) -> &'static str {
                match self {
                    #(
                        Self::#id => #name,
                    )*
                }
            }

            /// Get the string code for this currency value.
            pub const fn is_fund(&self) -> bool {
                match self {
                    #(
                        Self::#id => #is_fund,
                    )*
                }
            }

            /// Get the minor unit decimal place, if there is a minor unit.
            pub const fn minor_unit(&self) -> Option<u8> {
                match self {
                    #(
                        Self::#id => #minor_unit,
                    )*
                }
            }
        }
    }
}

/// Actual code generation
pub(crate) fn try_generate(input: TokenStream) -> Result<TokenStream> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(|error| {
        let message = format!("CARGO_MANIFEST_DIR variable not found: {error}");
        Error::new_spanned(&input, message)
    })?;

    let Config { xml, span } = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(input)
        .and_then(|args| Config::build(&manifest_dir, &args))?;

    let file = File::open(&xml).map_err(|error| {
        let message = format!("Could not open `xml` file: {error}");
        Error::new(span, message)
    })?;

    let reader = BufReader::new(file);

    let doc = de::from_reader::<_, CurrencyDoc>(reader).map_err(|error| {
        let message = format!("Could not parse XML file: {error}");
        Error::new(span, message)
    })?;

    let mut entries = doc.table().entries().to_vec();
    entries.sort_by_cached_key(CurrencyEntry::number);

    let entryset = EntrySet::from_entries(&entries);

    let mut retval = build_error();
    retval.extend(build_enum(&entryset));
    retval.extend(build_impl(&entryset));

    Ok(retval)
}
