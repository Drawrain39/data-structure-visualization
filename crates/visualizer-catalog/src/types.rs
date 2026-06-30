use serde::{Deserialize, Serialize};
pub use visualizer_core::AlgorithmCategory;

// ── Types ──────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Complexity {
    pub time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worst: Option<String>,
    pub space: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlgorithmMeta {
    pub key: String,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    pub category: AlgorithmCategory,
    pub description: String,
    pub complexity: Complexity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable: Option<String>,
    pub use_cases: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSample {
    pub language: String,
    pub label: String,
    pub lines: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSamples {
    pub cpp: CodeSample,
    pub python: CodeSample,
    pub rust: CodeSample,
}

pub type LineMap = std::collections::BTreeMap<String, Vec<usize>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlgorithmEntry {
    pub algorithm: String,
    pub meta: AlgorithmMeta,
    pub samples: CodeSamples,
    pub line_map: LineMap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub key: String,
    pub label: String,
}
