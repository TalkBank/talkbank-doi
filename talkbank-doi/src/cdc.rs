//! Parser for `0metadata.cdc` files.
//!
//! Extracts ALL metadata fields, not just the 5 the old system used.

use anyhow::{Context, Result, bail};
use std::path::Path;

/// All metadata from a `0metadata.cdc` file.
#[derive(Debug, Clone, Default)]
pub struct CorpusMetadata {
    // Required
    pub title: Option<String>,
    pub creators: Vec<String>,
    pub date: Option<String>,

    // Identification
    pub doi: Option<String>,
    pub cmdi_pid: Option<String>,

    // Rich metadata
    pub language: Option<String>,
    pub description: Option<String>,
    pub country: Option<String>,
    pub subjects: Vec<String>,
    pub publisher: Option<String>,

    // OLAC metadata
    pub olac_linguistic_field: Option<String>,
    pub olac_discourse_type: Option<String>,
    pub olac_linguistic_type: Option<String>,
    pub olac_languages: Vec<String>,

    // IMDI metadata (preserved as key-value pairs)
    pub imdi: Vec<(String, String)>,

    // Other
    pub contributors: Vec<String>,
    pub resource_type: Option<String>,
}

impl CorpusMetadata {
    /// Extract the publication year from the date field.
    pub fn publication_year(&self) -> Option<u32> {
        self.date.as_ref().and_then(|d| {
            d.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .ok()
        })
    }

    /// Validate that required fields are present.
    pub fn validate(&self, path: &Path) -> Result<()> {
        if self.title.is_none() {
            bail!("{}: missing Title", path.display());
        }
        if self.creators.is_empty() {
            bail!("{}: missing Creator", path.display());
        }
        if self.date.is_none() {
            bail!("{}: missing Date", path.display());
        }
        Ok(())
    }
}

/// Parse a `0metadata.cdc` file, extracting all fields.
pub fn parse(path: &Path) -> Result<CorpusMetadata> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;

    let mut meta = CorpusMetadata::default();

    for line in content.lines() {
        let line = line.trim_end();

        // Skip empty lines and @-prefixed headers
        if line.is_empty() || line.starts_with("@UTF8") || line.starts_with("@Window:") {
            continue;
        }

        // Parse "Key:\tValue" or "Key:\t\tValue" (some files have double tabs)
        let Some((key, value)) = split_field(line) else {
            continue;
        };
        let value = value.trim();

        match key {
            "Title" => {
                if meta.title.is_none() && !value.is_empty() {
                    meta.title = Some(value.to_string());
                }
            }
            "Creator" => {
                if !value.is_empty() {
                    meta.creators.push(value.to_string());
                }
            }
            "Date" => {
                if meta.date.is_none() && !value.is_empty() {
                    meta.date = Some(value.to_string());
                }
            }
            "DOI" => {
                if !value.is_empty() {
                    // Strip optional "doi:" prefix
                    let doi = value.strip_prefix("doi:").unwrap_or(value);
                    meta.doi = Some(doi.trim().to_string());
                }
            }
            "CMDI_PID" => {
                if !value.is_empty() {
                    meta.cmdi_pid = Some(value.to_string());
                }
            }
            "Language" => {
                if meta.language.is_none() && !value.is_empty() {
                    meta.language = Some(value.to_string());
                }
            }
            "Description" => {
                if meta.description.is_none() && !value.is_empty() {
                    meta.description = Some(value.to_string());
                }
            }
            "Country" => {
                if meta.country.is_none() && !value.is_empty() {
                    meta.country = Some(value.to_string());
                }
            }
            "Subject" => {
                if !value.is_empty() {
                    meta.subjects.push(value.to_string());
                }
            }
            "Publisher" => {
                if meta.publisher.is_none() && !value.is_empty() {
                    meta.publisher = Some(value.to_string());
                }
            }
            "Contributor" => {
                if !value.is_empty() {
                    meta.contributors.push(value.to_string());
                }
            }
            "Type" => {
                if meta.resource_type.is_none() && !value.is_empty() {
                    meta.resource_type = Some(value.to_string());
                }
            }
            _ if key.starts_with("Subject.olac:linguistic-field") => {
                if !value.is_empty() {
                    meta.olac_linguistic_field = Some(value.to_string());
                }
            }
            _ if key.starts_with("Subject.olac:discourse-type")
                || key.starts_with("Type.olac:discourse-type") =>
            {
                if meta.olac_discourse_type.is_none() && !value.is_empty() {
                    meta.olac_discourse_type = Some(value.to_string());
                }
            }
            _ if key.starts_with("Type.olac:linguistic-type") => {
                if !value.is_empty() {
                    meta.olac_linguistic_type = Some(value.to_string());
                }
            }
            _ if key.starts_with("Subject.olac:language") => {
                if !value.is_empty() {
                    meta.olac_languages.push(value.to_string());
                }
            }
            _ if key.starts_with("IMDI_") => {
                if !value.is_empty() {
                    let imdi_key = key.strip_prefix("IMDI_").unwrap_or(key);
                    meta.imdi.push((imdi_key.to_string(), value.to_string()));
                }
            }
            _ => {
                // Unknown field — skip silently
            }
        }
    }

    Ok(meta)
}

/// Write a DOI value back into a `0metadata.cdc` file.
pub fn write_doi(path: &Path, doi: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;

    let doi_line = format!("DOI:\tdoi:{doi}");
    let mut found = false;
    let new_content: String = content
        .lines()
        .map(|line| {
            if line.starts_with("DOI:") {
                found = true;
                doi_line.clone()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let final_content = if found {
        new_content
    } else {
        format!("{}\n{}\n", content.trim_end(), doi_line)
    };

    std::fs::write(path, final_content)
        .with_context(|| format!("writing {}", path.display()))?;

    Ok(())
}

/// Split a "Key:\tValue" line into (key, value).
fn split_field(line: &str) -> Option<(&str, &str)> {
    let (key, rest) = line.split_once(':')?;
    let value = rest.trim_start_matches('\t').trim_start();
    Some((key.trim(), value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_field() {
        assert_eq!(split_field("Title:\tFoo"), Some(("Title", "Foo")));
        assert_eq!(split_field("Date:\t\t2022"), Some(("Date", "2022")));
        assert_eq!(split_field("DOI:\tdoi:10.21415/XXXX"), Some(("DOI", "doi:10.21415/XXXX")));
    }

    #[test]
    fn test_publication_year() {
        let mut meta = CorpusMetadata::default();
        meta.date = Some("2022-03-30".to_string());
        assert_eq!(meta.publication_year(), Some(2022));

        meta.date = Some("2004".to_string());
        assert_eq!(meta.publication_year(), Some(2004));
    }
}
