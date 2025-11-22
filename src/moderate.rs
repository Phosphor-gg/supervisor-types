use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModerationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_labels: Option<Vec<ModerationLabel>>,
    #[serde(default)]
    pub include_context: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchModerationRequest {
    pub texts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModerationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_labels: Option<Vec<ModerationLabel>>,
    #[serde(default)]
    pub include_context: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResponse {
    pub flagged: bool,
    pub labels: Vec<ModerationLabel>,
    pub scores: std::collections::HashMap<ModerationLabel, f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_context: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl Display for ModerationModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModerationModel::Observer => write!(f, "observer"),
            ModerationModel::Sentinel => write!(f, "sentinel"),
            ModerationModel::Arbiter => write!(f, "arbiter"),
        }
    }
}

impl Display for ModerationLabel {
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

impl ModerationModel {
    pub fn all_models() -> Vec<ModerationModel> {
        vec![
            ModerationModel::Observer,
            ModerationModel::Sentinel,
            ModerationModel::Arbiter,
        ]
    }

    pub fn to_name(&self) -> &str {
        match self {
            ModerationModel::Observer => "Observer",
            ModerationModel::Sentinel => "Sentinel",
            ModerationModel::Arbiter => "Arbiter",
        }
    }

    pub fn credits_per_byte(&self) -> i64 {
        match self {
            ModerationModel::Observer => 1,
            ModerationModel::Sentinel => 3,
            ModerationModel::Arbiter => 9,
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
