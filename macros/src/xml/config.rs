//! Configuration

use proc_macro2::Span;
use std::path::PathBuf;
use syn::{Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, Token, punctuated::Punctuated};

pub(crate) struct Config {
    /// The path to the XML file
    pub xml: PathBuf,
    /// The span of the xml file path.
    pub span: Span,
    /// The feature name to use for zerocopy.
    pub zerocopy: Option<String>,
}

impl Config {
    /// Parse the literal provided as a path to an XML file
    fn parse_xml(&mut self, manifest_dir: &str, lit: &Lit) -> Result<()> {
        if !self.xml.as_os_str().is_empty() {
            return Err(Error::new_spanned(lit, "Multiple `xml` parameters."));
        }

        match lit {
            Lit::Str(lit_str) => {
                self.xml.push(manifest_dir);
                self.xml.push("src");
                self.xml.push(lit_str.value());

                self.span = lit_str.span();
            }
            val => {
                return Err(Error::new_spanned(
                    val,
                    "`xml` must be a path to an XML file relative to the `src` dir of the calling crate",
                ));
            }
        }

        Ok(())
    }

    /// Parse the literal provided as a zerocopy string (or boolean to enable it)
    fn parse_zerocopy(&mut self, lit: &Lit) -> Result<()> {
        if self.zerocopy.is_some() {
            return Err(Error::new_spanned(lit, "Multiple `zerocopy` parameters."));
        }

        match lit {
            Lit::Bool(lit_bool) => {
                if lit_bool.value() {
                    self.zerocopy = Some("zerocopy".to_owned());
                }
            }
            Lit::Str(lit_str) => {
                self.zerocopy = Some(lit_str.value());
            }
            val => {
                return Err(Error::new_spanned(
                    val,
                    "`zerocopy` must be a string-literal feature name",
                ));
            }
        }

        Ok(())
    }

    /// Parse a namevalue token pair
    fn parse_namevalue(&mut self, manifest_dir: &str, tokens: &MetaNameValue) -> Result<()> {
        let ident = tokens
            .path
            .get_ident()
            .ok_or_else(|| syn::Error::new_spanned(tokens, "Must have specified ident"))?
            .to_string()
            .to_lowercase();
        let lit = match &tokens.value {
            Expr::Lit(ExprLit { lit, .. }) => lit,
            expr => return Err(Error::new_spanned(expr, "Must be a literal")),
        };

        match ident.as_str() {
            "xml" => self.parse_xml(manifest_dir, lit),
            "zerocopy" => self.parse_zerocopy(lit),
            other => {
                let message = format!(
                    "{other} is not a valid paramter. The only valid parameter is `xml`, which should refer to a file relative to the calling crate's `src` directory."
                );
                Err(Error::new_spanned(ident, message))
            }
        }
    }

    pub(crate) fn build(manifest_dir: &str, args: &Punctuated<Meta, Token![,]>) -> Result<Self> {
        let mut retval = Self {
            xml: PathBuf::default(),
            span: Span::call_site(),
            zerocopy: None,
        };

        for arg in args {
            match arg {
                Meta::Path(tokens) => {
                    return Err(Error::new_spanned(
                        tokens,
                        concat!(
                            "The only valid parameters are `xml`, which should refer to a file ",
                            "relative to the calling crate's `src` directory, and `zerocopy`, ",
                            "which should refer to the feature name for enabling zerocopy traits."
                        ),
                    ));
                }
                Meta::List(tokens) => {
                    return Err(Error::new_spanned(
                        tokens,
                        concat!(
                            "The only valid parameter is `xml`, which should refer to a file ",
                            "relative to the calling crate's `src` directory, and `zerocopy`, ",
                            "which should refer to the feature name for enabling zerocopy traits."
                        ),
                    ));
                }
                Meta::NameValue(tokens) => retval.parse_namevalue(manifest_dir, tokens)?,
            }
        }

        Ok(retval)
    }
}
