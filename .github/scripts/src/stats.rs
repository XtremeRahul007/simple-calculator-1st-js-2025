use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct LanguageStats {
    #[serde(rename = "nFiles")]
    pub n_files: u32,
    pub blank: u32,
    pub comment: u32,
    pub code: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Header {
    pub n_files: u32,
    pub n_lines: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClocStats {
    pub header: Header,
    #[serde(flatten)]
    pub languages: HashMap<String, LanguageStats>,
}

#[derive(Debug, Clone)]
pub struct LanguageEntry {
    pub name: String,
    pub stats: LanguageStats,
    pub percentage: f64,
}

pub fn parse_cloc_json(json: &str) -> Result<ClocStats, serde_json::Error> {
    let value: Value = serde_json::from_str(json)?;

    let header: Header = serde_json::from_value(value["header"].clone())?;

    let mut languages = HashMap::new();

    if let Value::Object(entries) = &value {
        for (name, stats) in entries {
            if name == "header" || name == "SUM" {
                continue;
            }

            let stats: LanguageStats = serde_json::from_value(stats.clone())?;

            languages.insert(name.clone(), stats);
        }
    }

    Ok(ClocStats { header, languages })
}

pub fn process_languages(
    languages: HashMap<String, LanguageStats>,
    max_rows: usize,
) -> Vec<LanguageEntry> {
    let total_code: u32 = languages.values().map(|s| s.code).sum();

    let mut entries: Vec<LanguageEntry> = languages
        .into_iter()
        .map(|(name, stats)| LanguageEntry {
            name,
            percentage: if total_code == 0 {
                0.0
            } else {
                stats.code as f64 / total_code as f64 * 100.0f64
            },
            stats,
        })
        .collect();

    entries.sort_by_key(|entry| std::cmp::Reverse(entry.stats.code));

    if entries.len() > max_rows {
        let remaining = entries.split_off(max_rows - 1);

        let other_stats = LanguageStats {
            n_files: remaining.iter().map(|e| e.stats.n_files).sum(),
            blank: remaining.iter().map(|e| e.stats.blank).sum(),
            comment: remaining.iter().map(|e| e.stats.comment).sum(),
            code: remaining.iter().map(|e| e.stats.code).sum(),
        };

        let other_percentage = if total_code == 0 {
            0.0
        } else {
            other_stats.code as f64 / total_code as f64 * 100.0f64
        };

        entries.push(LanguageEntry {
            name: "Other".to_string(),
            stats: other_stats,
            percentage: other_percentage,
        });
    }
    entries
}
