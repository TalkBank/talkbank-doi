//! Strongly typed DOI newtype and DataCite lifecycle state enum.
#![allow(dead_code)]

use std::fmt;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// A validated Digital Object Identifier in canonical form (no `doi:` prefix).
///
/// DOIs are case-insensitive by spec. We preserve the original case as received
/// from DataCite rather than force-casing, since DataCite treats them as
/// case-insensitive for lookup but returns them in their registered form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Doi(String);

impl Doi {
    /// Parse and validate a DOI string.
    ///
    /// Accepts bare DOIs (`10.21415/XXX`) or with the `doi:` scheme prefix.
    /// The DOI syntax is: `10.{registrant}/{suffix}` where registrant is
    /// one or more dot-separated groups of digits (ISO 26324).
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        // Strip optional scheme prefix (case-insensitive)
        let s = if let Some(rest) = s.strip_prefix("doi:").or_else(|| s.strip_prefix("DOI:")) {
            rest.trim()
        } else {
            s
        };

        let Some((prefix, suffix)) = s.split_once('/') else {
            bail!("invalid DOI {:?}: missing '/'", s);
        };
        if !prefix.starts_with("10.") {
            bail!("invalid DOI {:?}: registrant prefix must start with '10.'", s);
        }
        let registrant = &prefix[3..];
        if registrant.is_empty()
            || !registrant
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.')
        {
            bail!(
                "invalid DOI {:?}: registrant must be digits (got {:?})",
                s,
                registrant
            );
        }
        if suffix.is_empty() {
            bail!("invalid DOI {:?}: empty suffix", s);
        }

        Ok(Doi(s.to_string()))
    }

    /// Return the DOI string without any prefix.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return the DOI with `doi:` scheme prefix, as stored in CDC files.
    pub fn with_scheme(&self) -> String {
        format!("doi:{}", self.0)
    }

    /// Return the canonical HTTPS resolution URL for this DOI.
    pub fn resolve_url(&self) -> String {
        format!("https://doi.org/{}", self.0)
    }

    /// Case-insensitive equality check.
    pub fn eq_ignore_case(&self, other: &Doi) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }

    /// Return an uppercase version for use as a HashMap key.
    pub fn to_key(&self) -> String {
        self.0.to_ascii_uppercase()
    }
}

impl fmt::Display for Doi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<&str> for Doi {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        Doi::parse(s)
    }
}

/// The lifecycle state of a DOI in DataCite.
///
/// Transitions:
///   draft → registered (event: "register")
///   draft → findable   (event: "publish")
///   registered → findable (event: "publish")
///   findable → registered (event: "hide")   ← "retire"
///   draft → deleted    (DELETE request — only state that allows deletion)
///
/// Once findable, a DOI can never be deleted and can never go back to draft.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DoiState {
    #[default]
    Draft,
    /// Has metadata + URL. Resolves at doi.org. NOT indexed in DataCite search.
    Registered,
    /// Fully public. Resolves + indexed in DataCite search and Google Dataset Search.
    Findable,
}

impl DoiState {
    /// True if this DOI can be permanently deleted.
    pub fn can_delete(&self) -> bool {
        matches!(self, DoiState::Draft)
    }

    /// True if this DOI appears in DataCite search results.
    pub fn is_indexed(&self) -> bool {
        matches!(self, DoiState::Findable)
    }

    /// The DataCite API `event` value needed to hide this DOI from public discovery.
    /// Returns `None` if it's already hidden or a draft (use DELETE for drafts).
    pub fn retire_event(&self) -> Option<&'static str> {
        match self {
            DoiState::Findable => Some("hide"),
            DoiState::Registered | DoiState::Draft => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            DoiState::Draft => "draft",
            DoiState::Registered => "registered",
            DoiState::Findable => "findable",
        }
    }
}

impl fmt::Display for DoiState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bare() {
        let d = Doi::parse("10.21415/T5HK5G").unwrap();
        assert_eq!(d.as_str(), "10.21415/T5HK5G");
    }

    #[test]
    fn parse_with_scheme() {
        let d = Doi::parse("doi:10.21415/T5HK5G").unwrap();
        assert_eq!(d.as_str(), "10.21415/T5HK5G");
    }

    #[test]
    fn parse_with_uppercase_scheme() {
        let d = Doi::parse("DOI:10.21415/T5HK5G").unwrap();
        assert_eq!(d.as_str(), "10.21415/T5HK5G");
    }

    #[test]
    fn reject_no_slash() {
        assert!(Doi::parse("10.21415").is_err());
    }

    #[test]
    fn reject_bad_prefix() {
        assert!(Doi::parse("11.21415/X").is_err());
    }

    #[test]
    fn resolve_url() {
        let d = Doi::parse("10.21415/T5HK5G").unwrap();
        assert_eq!(d.resolve_url(), "https://doi.org/10.21415/T5HK5G");
    }

    #[test]
    fn doi_state_retire_event() {
        assert_eq!(DoiState::Findable.retire_event(), Some("hide"));
        assert_eq!(DoiState::Registered.retire_event(), None);
        assert_eq!(DoiState::Draft.retire_event(), None);
    }
}
