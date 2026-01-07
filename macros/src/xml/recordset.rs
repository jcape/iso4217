//! Struct-of-Arrays data set

use heck::ToPascalCase;
use iso4217_parser::CurrencyEntry;
use proc_macro2::{Span, TokenStream};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use syn::{Ident, LitByteStr};

#[derive(Clone, Default)]
pub(crate) struct EntrySet {
    /// The documentation string.
    doc: Vec<String>,

    /// The enumeration variant identifier.
    ident: Vec<Ident>,

    /// The ASCII code.
    currency: Vec<String>,

    /// The ASCII code, as bytes.
    bytes: Vec<LitByteStr>,

    /// The numeric code.
    number: Vec<u16>,

    /// The name of the currency.
    name: Vec<String>,

    /// Whether a value is a fund or not.
    is_fund: Vec<bool>,

    /// The number of decimal places in the minor unit.
    minor_unit: Vec<TokenStream>,

    /// A list of country names indexed by the numeric code of the currency they use.
    countries_by_number: BTreeMap<u16, BTreeSet<String>>,
}

impl EntrySet {
    /// Build an entry set from a slice of entries
    pub(crate) fn from_entries(entries: &[CurrencyEntry]) -> Self {
        let mut retval = Self::default();

        for entry in entries {
            if let Some(currency) = entry.currency()
                && let Some(number) = entry.number()
                && let Some(name) = entry.name()
            {
                let mut is_new = false;

                retval
                    .countries_by_number
                    .entry(number)
                    .and_modify(|countries| {
                        countries.insert(entry.country().to_owned());
                    })
                    .or_insert_with(|| {
                        is_new = true;
                        BTreeSet::from_iter([entry.country().to_owned()])
                    });

                if is_new {
                    let fund_str = if name.is_fund() { ", Fund" } else { "" };

                    let doc = format!(" {} ({currency}, {number}{fund_str})", name.name());
                    let id = currency.trim().to_pascal_case();
                    assert!(
                        id.is_ascii(),
                        "Invalid non-ASCII enum variant: {id} {number}"
                    );
                    let ident = quote::format_ident!("{id}");
                    let minor_unit = if let Some(unit) = entry.minor_unit() {
                        quote::quote! { Some(#unit) }
                    } else {
                        quote::quote! { None }
                    };

                    let bytes = LitByteStr::new(currency.as_bytes(), Span::mixed_site());

                    retval.doc.push(doc);
                    retval.ident.push(ident);
                    retval.currency.push(currency.to_owned());
                    retval.bytes.push(bytes);
                    retval.number.push(number);
                    retval.is_fund.push(name.is_fund());
                    retval.name.push(name.name().to_owned());
                    retval.minor_unit.push(minor_unit);
                }
            }
        }

        retval
    }

    /// The documentation strings.
    pub(crate) fn doc(&self) -> &[String] {
        &self.doc
    }

    /// The enum identifiers.
    pub(crate) fn ident(&self) -> &[Ident] {
        &self.ident
    }

    /// The string value
    pub(crate) fn currency(&self) -> &[String] {
        &self.currency
    }

    /// The string value, as bytes.
    pub(crate) fn bytes(&self) -> &[LitByteStr] {
        &self.bytes
    }

    /// The numeric value.
    pub(crate) fn number(&self) -> &[u16] {
        &self.number
    }

    /// The fund status.
    pub(crate) fn is_fund(&self) -> &[bool] {
        &self.is_fund
    }

    /// The number of decimal places in the minor unit.
    pub(crate) fn minor_unit(&self) -> &[TokenStream] {
        &self.minor_unit
    }

    /// The name of the currency.
    pub(crate) fn name(&self) -> &[String] {
        &self.name
    }
}

impl Debug for EntrySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntrySet")
            .field("doc", &self.doc)
            .field("ident", &self.ident)
            .field("currency", &self.currency)
            .field("bytes", &"...")
            .field("number", &self.number)
            .field("name", &self.name)
            .field("is_fund", &self.is_fund)
            .field("minor_unit", &self.minor_unit)
            .field("countries_by_number", &self.countries_by_number)
            .finish()
    }
}
