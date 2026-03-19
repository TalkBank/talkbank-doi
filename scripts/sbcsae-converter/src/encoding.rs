use crate::diagnostics::Diagnostics;
use crate::types::DiagnosticCode;

/// Read a file as raw bytes and decode from ISO-8859-1, repairing known
/// encoding issues (NUL bytes, Windows-1252 smart quotes).
pub fn read_and_decode(path: &std::path::Path, diag: &mut Diagnostics) -> std::io::Result<String> {
    let bytes = std::fs::read(path)?;

    // Decode as ISO-8859-1 (single-byte, never fails).
    let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&bytes);
    let text = decoded.into_owned();

    // Repair NUL bytes (0x00 → 'c') — 5 files have these replacing 'c'/'C'.
    let mut line_number = 1;
    let mut repaired = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\0' => {
                repaired.push('c');
                diag.warn(
                    line_number,
                    None,
                    DiagnosticCode::NulByte,
                    "NUL byte (0x00) replaced with 'c'",
                );
            }
            // Windows-1252 right single quote (0x92 → U+2019) decoded by encoding_rs.
            // Map to ASCII apostrophe for consistency.
            '\u{2019}' => {
                repaired.push('\'');
                diag.warn(
                    line_number,
                    None,
                    DiagnosticCode::Windows1252Char,
                    "Windows-1252 right single quote (0x92) mapped to ASCII apostrophe",
                );
            }
            '\n' => {
                repaired.push('\n');
                line_number += 1;
            }
            '\r' => {
                // Strip \r (CRLF → LF). Don't push.
            }
            _ => {
                repaired.push(ch);
            }
        }
    }

    Ok(repaired)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nul_byte_replaced() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"the \x00hurch").unwrap();
        let mut diag = Diagnostics::new();
        let text = read_and_decode(tmp.path(), &mut diag).unwrap();
        assert_eq!(text, "the church");
        assert_eq!(diag.len(), 1);
    }

    #[test]
    fn windows_1252_smart_quote() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"don\x92t").unwrap();
        let mut diag = Diagnostics::new();
        let text = read_and_decode(tmp.path(), &mut diag).unwrap();
        assert_eq!(text, "don't");
        assert_eq!(diag.len(), 1);
    }

    #[test]
    fn crlf_stripped() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"line1\r\nline2\r\n").unwrap();
        let mut diag = Diagnostics::new();
        let text = read_and_decode(tmp.path(), &mut diag).unwrap();
        assert_eq!(text, "line1\nline2\n");
        assert_eq!(diag.len(), 0);
    }
}
