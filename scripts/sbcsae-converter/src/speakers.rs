use std::collections::BTreeMap;

use crate::diagnostics::Diagnostics;
use crate::types::DiagnosticCode;

/// Maximum CHAT speaker ID length (Brian's rule).
const MAX_WHO: usize = 4;

/// Build TRN-name → CHAT-ID mapping, replicating the Java `Speakers` logic.
///
/// Speakers are mapped in order of first appearance. Special cases from the
/// original Java code take priority over the default truncation rule.
pub fn build_speaker_map(speakers_in_order: &[String], diag: &mut Diagnostics) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let mut id_to_name: BTreeMap<String, String> = BTreeMap::new();
    let mut junk_counter = 0;

    for name in speakers_in_order {
        if map.contains_key(name) {
            continue;
        }

        let mut id = create_id(name);

        // Check for conflicts.
        if let Some(existing_name) = id_to_name.get(&id) {
            if existing_name != name {
                let junk_id = format!("JUNK{junk_counter}");
                junk_counter += 1;
                diag.warn(
                    0,
                    None,
                    DiagnosticCode::SpeakerMapConflict,
                    format!(
                        "Speaker '{name}' truncates to '{id}' which conflicts with '{existing_name}'; using '{junk_id}'"
                    ),
                );
                id = junk_id;
            }
        }

        id_to_name.insert(id.clone(), name.clone());
        map.insert(name.clone(), id);
    }

    map
}

fn create_id(name: &str) -> String {
    // Strip > prefix for environment speakers.
    let name = name.trim_start_matches('>');

    // Special cases from Speakers.java.
    match name {
        "TOM_1" => return "TOM1".to_string(),
        "TOM_2" => return "TOM2".to_string(),
        "TOM_3" => return "TOM3".to_string(),
        "AUD_1" => return "AUD1".to_string(),
        "AUD_2" => return "AUD2".to_string(),
        "AUD_3" => return "AUD3".to_string(),
        "SHANE" => return "SHAN".to_string(),
        "SHARON" => return "SHA".to_string(),
        "KEN" => return "KEN".to_string(),
        "KENDRA" => return "KEND".to_string(),
        _ => {}
    }

    // Default: truncate to MAX_WHO characters.
    let end = name.len().min(MAX_WHO);
    name[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_truncation() {
        assert_eq!(create_id("JAMIE"), "JAMI");
        assert_eq!(create_id("HAROLD"), "HARO");
        assert_eq!(create_id("MILES"), "MILE");
        assert_eq!(create_id("PETE"), "PETE");
        assert_eq!(create_id("AL"), "AL");
    }

    #[test]
    fn special_cases() {
        assert_eq!(create_id("SHANE"), "SHAN");
        assert_eq!(create_id("SHARON"), "SHA");
        assert_eq!(create_id("TOM_1"), "TOM1");
        assert_eq!(create_id("KENDRA"), "KEND");
        assert_eq!(create_id("KEN"), "KEN");
    }

    #[test]
    fn environment_speaker() {
        assert_eq!(create_id(">ENV"), "ENV");
        assert_eq!(create_id(">DOG"), "DOG");
    }

    #[test]
    fn conflict_detection() {
        let speakers = vec![
            "JAMES".to_string(),
            "JAMIE".to_string(), // Both truncate to JAMI... wait, JAMES→JAME, JAMIE→JAMI. No conflict.
        ];
        let mut diag = Diagnostics::new();
        let map = build_speaker_map(&speakers, &mut diag);
        assert_eq!(map["JAMES"], "JAME");
        assert_eq!(map["JAMIE"], "JAMI");
        assert_eq!(diag.len(), 0);
    }

    #[test]
    fn actual_conflict() {
        let speakers = vec![
            "ABCD".to_string(),
            "ABCDEF".to_string(), // Both truncate to ABCD.
        ];
        let mut diag = Diagnostics::new();
        let map = build_speaker_map(&speakers, &mut diag);
        assert_eq!(map["ABCD"], "ABCD");
        assert_eq!(map["ABCDEF"], "JUNK0");
        assert_eq!(diag.len(), 1);
    }
}
