use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationRequest {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<ModerationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_labels: Option<Vec<ModerationLabel>>,
    #[serde(default)]
    include_context: bool,
}
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

impl std::fmt::Display for ModerationModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModerationModel::Observer => write!(f, "Observer"),
            ModerationModel::Sentinel => write!(f, "Sentinel"),
            ModerationModel::Arbiter => write!(f, "Arbiter"),
        }
    }
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

impl FromStr for ModerationLabel {
    type Err = ();

    fn from_str(input: &str) -> Result<ModerationLabel, Self::Err> {
        match input {
            "S" => Ok(ModerationLabel::S),
            "H" => Ok(ModerationLabel::H),
            "V" => Ok(ModerationLabel::V),
            "HR" => Ok(ModerationLabel::HR),
            "SH" => Ok(ModerationLabel::SH),
            "S3" => Ok(ModerationLabel::S3),
            "SP" => Ok(ModerationLabel::SP),
            "SE" => Ok(ModerationLabel::SE),
            "T" => Ok(ModerationLabel::T),
            _ => Err(()),
        }
    }
}

impl FromStr for ModerationModel {
    type Err = ();

    fn from_str(input: &str) -> Result<ModerationModel, Self::Err> {
        match input.to_lowercase().as_str() {
            "observer" => Ok(ModerationModel::Observer),
            "sentinel" => Ok(ModerationModel::Sentinel),
            "arbiter" => Ok(ModerationModel::Arbiter),
            _ => Err(()),
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
