//! ISO 4217 XML Parser

use chrono::{NaiveDate, ParseResult};
use serde::{Deserialize, Serialize};

/// The currency document
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CurrencyDoc {
    /// The table of currency entries.
    #[serde(alias = "CcyTbl")]
    table: CurrencyTable,

    /// The date this document was published.
    #[serde(alias = "@Pblshd")]
    published: String,
}

impl CurrencyDoc {
    /// The table contained within this document.
    #[must_use]
    pub fn table(&self) -> &CurrencyTable {
        &self.table
    }

    /// The date this document was published.
    ///
    /// # Errors
    ///
    /// - [`ParseError`](chrono::format::ParseError) when the date string is not in the format
    ///   `YYYY-MM-DD`.
    pub fn published(&self) -> ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(&self.published, "%Y-%m-%d")
    }
}

/// The currency table
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CurrencyTable {
    /// The individual currency entries.
    #[serde(alias = "CcyNtry")]
    entries: Vec<CurrencyEntry>,
}

impl CurrencyTable {
    /// Retrieve a slice of the entries in this table.
    #[must_use]
    pub fn entries(&self) -> &[CurrencyEntry] {
        &self.entries
    }
}

/// An Currency XML Entry
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CurrencyEntry {
    /// The name of the country.
    #[serde(alias = "CtryNm")]
    country: String,

    /// The name of the currency.
    #[serde(alias = "CcyNm")]
    name: Option<CurrencyName>,

    /// The 3-character currency code.
    #[serde(alias = "Ccy")]
    currency: Option<String>,

    /// The numeric currency code.
    #[serde(alias = "CcyNbr")]
    number: Option<u16>,

    /// The minor unit decimal places.
    #[serde(alias = "CcyMnrUnts")]
    minor_unit: Option<String>,
}

impl CurrencyEntry {
    /// The country name.
    #[must_use]
    pub fn country(&self) -> &str {
        self.country.trim()
    }

    /// The currency name.
    #[must_use]
    pub fn name(&self) -> Option<&CurrencyName> {
        self.name.as_ref()
    }

    /// The currency code.
    ///
    /// This may be optional if the given country doesn't have universal currency.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        self.currency.as_deref().map(str::trim)
    }

    /// Retrieve the currency code as a number.
    #[must_use]
    pub fn number(&self) -> Option<u16> {
        self.number
    }

    /// Retrieve the minor unit decimal places, if applicable.
    #[must_use]
    pub fn minor_unit(&self) -> Option<u8> {
        self.minor_unit.as_deref().and_then(|mu| match mu.trim() {
            "N.A." | "" => None,
            other => other.parse::<u8>().ok(),
        })
    }
}

/// A currency name
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CurrencyName {
    /// Whether the currency is a fund or not.
    #[serde(alias = "@IsFund")]
    is_fund: Option<bool>,

    /// The currency name.
    #[serde(alias = "$value")]
    name: String,
}

impl CurrencyName {
    /// Whether the currency is a fund or not.
    #[must_use]
    pub fn is_fund(&self) -> bool {
        self.is_fund.unwrap_or_default()
    }

    /// The (trimmed) currency name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.trim()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quick_xml::de;
    use std::{fs::File, io::BufReader, path::PathBuf};

    const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
    const SRC_DIR: &str = "src";

    #[yare::parameterized(
        xml20260101 = { "2026-01-01.xml", 280 }
    )]
    fn counts(filename: &str, count: usize) {
        let mut path = PathBuf::from(BASE_PATH);
        path.push(SRC_DIR);
        path.push(filename);

        let file = File::open(path).expect("file");
        let reader = BufReader::new(file);

        let contents = de::from_reader::<_, CurrencyDoc>(reader).expect("XML reader");
        let entries = contents.table().entries();

        assert_eq!(count, entries.len());

        for entry in entries {
            if let Some(code) = entry.currency()
                && code == "USN"
            {
                assert!(entry.name().unwrap().is_fund());
            }
        }
    }
}
