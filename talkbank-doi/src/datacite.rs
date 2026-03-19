//! DataCite REST API types and client.
//!
//! Uses the modern REST API (<https://api.datacite.org>) with JSON,
//! not the legacy MDS API with XML.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::cdc::CorpusMetadata;

/// DataCite API configuration.
#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub prefix: String,
}

impl Config {
    /// Load from environment variables.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            api_url: std::env::var("DATACITE_API")
                .unwrap_or_else(|_| "https://api.datacite.org".to_string()),
            client_id: std::env::var("DATACITE_CLIENT_ID")
                .context("DATACITE_CLIENT_ID not set")?,
            client_secret: std::env::var("DATACITE_CLIENT_SECRET")
                .context("DATACITE_CLIENT_SECRET not set")?,
            prefix: std::env::var("DATACITE_PREFIX")
                .unwrap_or_else(|_| "10.21415".to_string()),
        })
    }
}

/// A DOI record as represented in the DataCite REST API.
#[derive(Debug, Serialize, Deserialize)]
pub struct DoiRecord {
    pub data: DoiData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoiData {
    #[serde(rename = "type")]
    pub record_type: String,
    pub attributes: DoiAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoiAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    pub creators: Vec<Creator>,
    pub titles: Vec<Title>,
    pub publisher: String,
    pub publication_year: u32,
    pub types: ResourceType,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub descriptions: Vec<Description>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub subjects: Vec<Subject>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub geo_locations: Vec<GeoLocation>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dates: Vec<DateEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceType {
    pub resource_type_general: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub description: String,
    pub description_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_scheme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    pub geo_location_place: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateEntry {
    pub date: String,
    pub date_type: String,
}

/// Build a DataCite DOI record from corpus metadata.
pub fn build_record(meta: &CorpusMetadata, target_url: &str, prefix: &str) -> Result<DoiRecord> {
    let title = meta.title.as_deref()
        .context("title required")?;
    let year = meta.publication_year()
        .context("publication year required")?;

    let creators: Vec<Creator> = meta.creators.iter()
        .map(|name| Creator { name: name.clone() })
        .collect();

    if creators.is_empty() {
        bail!("at least one creator required");
    }

    let mut subjects: Vec<Subject> = meta.subjects.iter()
        .map(|s| Subject { subject: s.clone(), subject_scheme: None })
        .collect();

    if let Some(ref field) = meta.olac_linguistic_field {
        subjects.push(Subject {
            subject: field.clone(),
            subject_scheme: Some("OLAC".to_string()),
        });
    }
    if let Some(ref dtype) = meta.olac_discourse_type {
        subjects.push(Subject {
            subject: dtype.clone(),
            subject_scheme: Some("OLAC".to_string()),
        });
    }

    let mut descriptions = Vec::new();
    if let Some(ref desc) = meta.description {
        descriptions.push(Description {
            description: desc.clone(),
            description_type: "Abstract".to_string(),
        });
    }

    let mut geo_locations = Vec::new();
    if let Some(ref country) = meta.country {
        geo_locations.push(GeoLocation {
            geo_location_place: country.clone(),
        });
    }

    let mut dates = Vec::new();
    if let Some(ref date) = meta.date {
        dates.push(DateEntry {
            date: date.clone(),
            date_type: "Issued".to_string(),
        });
    }

    Ok(DoiRecord {
        data: DoiData {
            record_type: "dois".to_string(),
            attributes: DoiAttributes {
                doi: meta.doi.clone(),
                prefix: if meta.doi.is_none() { Some(prefix.to_string()) } else { None },
                creators,
                titles: vec![Title {
                    title: title.to_string(),
                    lang: Some("en".to_string()),
                }],
                publisher: meta.publisher.clone()
                    .unwrap_or_else(|| "TalkBank".to_string()),
                publication_year: year,
                types: ResourceType {
                    resource_type_general: "Dataset".to_string(),
                },
                url: target_url.to_string(),
                language: meta.language.clone(),
                descriptions,
                subjects,
                geo_locations,
                dates,
            },
        },
    })
}

/// Mint a new DOI at DataCite. Returns the minted DOI string.
pub fn mint(client: &reqwest::blocking::Client, config: &Config, record: &DoiRecord) -> Result<String> {
    let url = format!("{}/dois", config.api_url);
    let resp = client
        .post(&url)
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .json(record)
        .send()
        .context("DataCite API request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        bail!("DataCite mint failed ({status}): {body}");
    }

    let result: DoiRecord = resp.json().context("parsing DataCite response")?;
    result.data.attributes.doi
        .context("DataCite response missing DOI")
}

/// Update an existing DOI's metadata at DataCite.
pub fn update(client: &reqwest::blocking::Client, config: &Config, doi: &str, record: &DoiRecord) -> Result<()> {
    let url = format!("{}/dois/{doi}", config.api_url);
    let resp = client
        .put(&url)
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .json(record)
        .send()
        .context("DataCite API request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        bail!("DataCite update failed for {doi} ({status}): {body}");
    }

    Ok(())
}

/// Query DataCite for an existing DOI's metadata.
pub fn query(client: &reqwest::blocking::Client, config: &Config, doi: &str) -> Result<DoiRecord> {
    let url = format!("{}/dois/{doi}", config.api_url);
    let resp = client
        .get(&url)
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .send()
        .context("DataCite API request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        bail!("DataCite query failed for {doi} ({status}): {body}");
    }

    resp.json().context("parsing DataCite response")
}
