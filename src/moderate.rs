use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerateResponse {
    pub flagged: bool,
    pub labels: Vec<ModerationLabel>,
    pub scores: std::collections::HashMap<ModerationLabel, f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_context: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    pub context_labels: Option<Vec<ModerationLabel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModerationLabel {
    S,
    H,
    V,
    HR,
    SH,
    S3,
    SP,
    SE,
    T,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ModerationModel {
    Observer,
    Sentinel,
    Arbiter,
}

impl std::fmt::Display for ModerationLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModerationLabel::S => write!(f, "S"),
            ModerationLabel::T => write!(f, "T"),
            ModerationLabel::H => write!(f, "H"),
            ModerationLabel::V => write!(f, "V"),
            ModerationLabel::HR => write!(f, "HR"),
            ModerationLabel::SH => write!(f, "SH"),
            ModerationLabel::S3 => write!(f, "S3"),
            ModerationLabel::SP => write!(f, "SP"),
            ModerationLabel::SE => write!(f, "SE"),
        }
    }
}

impl ModerationLabel {
    pub fn to_name(&self) -> &str {
        match self {
            ModerationLabel::S => "Sexual",
            ModerationLabel::H => "Harassment",
            ModerationLabel::V => "Violence",
            ModerationLabel::HR => "Hate/Racism",
            ModerationLabel::SH => "Self-Harm",
            ModerationLabel::S3 => "Sexual (Severe/Minors)",
            ModerationLabel::SP => "Spam",
            ModerationLabel::T => "Toxicity",
            ModerationLabel::SE => "Sensitive Content",
        }
    }

    pub fn all_labels() -> Vec<ModerationLabel> {
        vec![
            ModerationLabel::S,
            ModerationLabel::H,
            ModerationLabel::V,
            ModerationLabel::HR,
            ModerationLabel::SH,
            ModerationLabel::S3,
            ModerationLabel::SP,
            ModerationLabel::SE,
        ]
    }
}
