//! Lightweight CHAT file header extraction via line scanning and regex.
//!
//! Fully self-contained — no dependency on talkbank-model or talkbank-direct-parser.
//! Each `.cha` file is read once and scanned line-by-line for the small set of
//! headers relevant to media checking.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

// ── Local media types (replaces talkbank-model dependency) ──────────────────

/// Parsed `@Media` header payload.
#[derive(Clone, Debug)]
pub struct MediaHeader {
    /// Media basename without extension (may be quoted for remote URLs).
    pub filename: String,
    /// Capture modality.
    pub media_type: MediaType,
    /// Optional availability status.
    pub status: Option<MediaStatus>,
}

/// Media modality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
}

/// Optional media availability status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaStatus {
    Missing,
    Unlinked,
    Notrans,
}

impl MediaStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Unlinked => "unlinked",
            Self::Notrans => "notrans",
        }
    }
}

// ── Regexes ─────────────────────────────────────────────────────────────────

/// `@Media:\tfilename, audio|video[, missing|unlinked|notrans]`
static MEDIA_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?m)^@Media:\t([^" ,]+|"[^"]+") *, *(audio|video)(?: *, *(missing|unlinked|notrans))?"#,
    )
    .expect("media regex is valid")
});

/// `@ID:\tlang|corpus|speaker|...` — capture corpus (2nd pipe field).
static ID_CORPUS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^@ID:\t[^\|]*\|([^\|]*)").expect("id corpus regex is valid")
});

/// Timing bullets: `\u{0015}digits_digits\u{0015}`.
static BULLET_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\x15\d+_\d+\x15").expect("bullet regex is valid")
});

/// `%pic:"filename"` references.
static PIC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"%pic:\s*"([^"]+)""#).expect("pic regex is valid")
});

// ── Extracted info ──────────────────────────────────────────────────────────

/// Information extracted from a CHAT file, sufficient for media checking.
#[derive(Debug)]
pub struct ChatFileInfo {
    /// Absolute path to the `.cha` file.
    pub path: PathBuf,
    /// Parsed `@Media` header, if present.
    pub media: Option<MediaHeader>,
    /// Corpus names from all `@ID` headers.
    pub id_corpora: Vec<String>,
    /// Whether any timing bullets exist in the file.
    pub has_bullets: bool,
    /// Filenames referenced by `%pic:` directives.
    pub pic_refs: Vec<String>,
}

/// Extract media-relevant information from a single CHAT file.
///
/// Reads the file once, applies regexes for headers, bullets, and %pic refs.
pub fn extract_chat_info(path: &Path) -> Result<ChatFileInfo, ExtractError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ExtractError::Read { path: path.to_owned(), source: e })?;

    // @Media header (at most one per file).
    let media = MEDIA_RE.captures(&content).map(|cap| {
        let filename = cap[1].trim_matches('"').to_string();
        let media_type = match &cap[2] {
            "audio" => MediaType::Audio,
            "video" => MediaType::Video,
            _ => unreachable!("regex only matches audio|video"),
        };
        let status = cap.get(3).map(|m| match m.as_str() {
            "missing" => MediaStatus::Missing,
            "unlinked" => MediaStatus::Unlinked,
            "notrans" => MediaStatus::Notrans,
            _ => unreachable!("regex only matches missing|unlinked|notrans"),
        });
        MediaHeader { filename, media_type, status }
    });

    // @ID corpus fields (may be multiple).
    let id_corpora: Vec<String> = ID_CORPUS_RE
        .captures_iter(&content)
        .filter_map(|cap| {
            let corpus = cap[1].trim();
            if corpus.is_empty() { None } else { Some(corpus.to_string()) }
        })
        .collect();

    // Timing bullets anywhere in the file.
    let has_bullets = BULLET_RE.is_match(&content);

    // %pic references.
    let pic_refs: Vec<String> = PIC_RE
        .captures_iter(&content)
        .map(|cap| cap[1].to_string())
        .collect();

    Ok(ChatFileInfo {
        path: path.to_owned(),
        media,
        id_corpora,
        has_bullets,
        pic_refs,
    })
}

/// Errors during CHAT extraction.
#[derive(Debug, thiserror::Error)]
pub enum ExtractError {
    #[error("failed to read {path}: {source}")]
    Read { path: PathBuf, source: std::io::Error },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bullet_regex_matches() {
        assert!(BULLET_RE.is_match("hello \x152061689_2062652\x15"));
        assert!(!BULLET_RE.is_match("no bullets here"));
    }

    #[test]
    fn pic_regex_matches() {
        let text = r#"%pic:"foo.jpg" some text %pic:"bar.gif""#;
        let refs: Vec<String> = PIC_RE
            .captures_iter(text)
            .map(|c| c[1].to_string())
            .collect();
        assert_eq!(refs, vec!["foo.jpg", "bar.gif"]);
    }

    #[test]
    fn media_header_parsing() {
        let text = "@Media:\ttest_file, audio, unlinked\n";
        let cap = MEDIA_RE.captures(text).unwrap();
        assert_eq!(&cap[1], "test_file");
        assert_eq!(&cap[2], "audio");
        assert_eq!(cap.get(3).unwrap().as_str(), "unlinked");
    }

    #[test]
    fn media_header_no_status() {
        let text = "@Media:\tmy_recording, video\n";
        let cap = MEDIA_RE.captures(text).unwrap();
        assert_eq!(&cap[1], "my_recording");
        assert_eq!(&cap[2], "video");
        assert!(cap.get(3).is_none());
    }

    #[test]
    fn media_header_quoted_filename() {
        let text = "@Media:\t\"http://example.com/file\", audio\n";
        let cap = MEDIA_RE.captures(text).unwrap();
        assert_eq!(&cap[1], "\"http://example.com/file\"");
    }

    #[test]
    fn id_corpus_extraction() {
        let text = "@ID:\teng|MyCorpus|CHI|3;6.||||Target_Child|||\n\
                    @ID:\teng|MyCorpus|MOT|||||Mother|||\n";
        let corpora: Vec<String> = ID_CORPUS_RE
            .captures_iter(text)
            .map(|c| c[1].to_string())
            .collect();
        assert_eq!(corpora, vec!["MyCorpus", "MyCorpus"]);
    }

    #[test]
    fn id_corpus_empty() {
        let text = "@ID:\teng||CHI|3;6.||||Target_Child|||\n";
        let corpora: Vec<String> = ID_CORPUS_RE
            .captures_iter(text)
            .filter_map(|c| {
                let s = c[1].trim();
                if s.is_empty() { None } else { Some(s.to_string()) }
            })
            .collect();
        assert!(corpora.is_empty());
    }
}
