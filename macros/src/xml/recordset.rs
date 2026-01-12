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

    /// The iso3166 country identifier.
    country_ident: Vec<Ident>,

    /// Currency number by country ident.
    currency_ident: Vec<Ident>,
}

const NON_COUNTRIES: &[&str] = &[
    "ArabMonetaryFund",
    "EuropeanUnion",
    "InternationalMonetaryFundImf",
    "MemberCountriesOfTheAfricanDevelopmentBankGroup",
    "SistemaUnitarioDeCompensacionRegionalDePagosSucre",
];

impl EntrySet {
    /// Build an entry set from a slice of entries
    pub(crate) fn from_entries(entries: &[CurrencyEntry]) -> Self {
        let mut retval = Self::default();

        // collect a map of country strings to currency numbers
        let mut country_to_currency = BTreeMap::new();

        // Previously seen numbers.
        let mut numbers = BTreeSet::default();

        for entry in entries {
            if let Some(currency) = entry.currency()
                && let Some(number) = entry.number()
                && let Some(name) = entry.name()
            {
                let mut id = name
                    .name()
                    .trim()
                    .to_pascal_case()
                    .replace("BolívarSoberano", "BolivarSoberano");

                // cleanup bad data
                if id == "BolivarSoberano" && currency == "VED" {
                    id.clear();
                    id.push_str("BolivarDigital");
                }
                assert!(
                    id.is_ascii(),
                    "Invalid non-ASCII enum variant: {id} {number}"
                );
                let ident = quote::format_ident!("{id}");

                let country_id = entry
                    .country()
                    .replace("(THE)", "")
                    .replace("(PLURINATIONAL STATE OF)", "")
                    .trim()
                    .to_pascal_case()
                    .replace("ÅlandIslands", "AlandIslands")
                    .replace("CôteDIvoire", "CoteDIvoire")
                    .replace("Curaçao", "Curacao")
                    .replace(
                        "CongoTheDemocraticRepublicOfThe",
                        "DemocraticRepublicOfTheCongo",
                    )
                    .replace("IranIslamicRepublicOf", "Iran")
                    .replace("KoreaTheDemocraticPeopleSRepublicOf", "NorthKorea")
                    .replace("KoreaTheRepublicOf", "SouthKorea")
                    .replace("LaoPeopleSDemocraticRepublic", "Laos")
                    .replace("MicronesiaFederatedStatesOf", "Micronesia")
                    .replace("MoldovaTheRepublicOf", "Moldova")
                    .replace("Réunion", "Reunion")
                    .replace("RussianFederation", "Russia")
                    .replace("SaintBarthélemy", "SaintBarthelemy")
                    .replace("SyrianArabRepublic", "Syria")
                    .replace("TaiwanProvinceOfChina", "Taiwan")
                    .replace("TanzaniaUnitedRepublicOf", "Tanzania")
                    .replace(
                        "UnitedKingdomOfGreatBritainAndNorthernIreland",
                        "UnitedKingdom",
                    )
                    .replace("Türki̇ye", "Turkey")
                    .replace("VenezuelaBolivarianRepublicOf", "Venezuela")
                    .replace("VirginIslandsBritish", "BritishVirginIslands");
                assert!(
                    country_id.is_ascii(),
                    "Invalid non-ASCII enum variant: {country_id} {number}"
                );

                if !country_id.starts_with("Zz")
                    && !NON_COUNTRIES.iter().any(|&val| val == country_id)
                    && !name.is_fund()
                {
                    let country_ident = Ident::new(&country_id, Span::mixed_site());
                    country_to_currency.insert(country_ident, ident.clone());
                }

                if numbers.insert(number) {
                    let fund_str = if name.is_fund() { ", Fund" } else { "" };
                    let doc = format!(" {} ({currency}, {number}{fund_str})", name.name());
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

        for (country_ident, currency_ident) in country_to_currency {
            retval.country_ident.push(country_ident);
            retval.currency_ident.push(currency_ident);
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

    /// A country identifier.
    ///
    /// This starts a separately indexed set of fields, alongside `currency_ident`. In particular,
    /// this field must match one of the enum variants from `iso3166-static`.
    pub(crate) fn country_ident(&self) -> &[Ident] {
        &self.country_ident
    }

    /// A currency identifier.
    ///
    /// This starts a separately indexed set of fields, alongside `country_ident`. In particular,
    /// this field will contain the Country code identifier for this currency.
    pub(crate) fn currency_ident(&self) -> &[Ident] {
        &self.currency_ident
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
            .field("country_ident", &self.country_ident)
            .field("currency_ident", &self.currency_ident)
            .finish()
    }
}
