use chrono::Utc;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationLogEntry {
    pub text: String,
    pub result: String,
    pub timestamp: String,
}

impl ModerationLogEntry {
    pub fn now(text: String, result: String) -> Self {
        Self {
            text,
            result,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
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
    Auto,
    Observer,
    Sentinel,
    Arbiter,
}

impl Display for ModerationModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModerationModel::Auto => write!(f, "auto"),
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
            "auto" => Ok(ModerationModel::Auto),
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
            ModerationModel::Auto,
            ModerationModel::Observer,
            ModerationModel::Sentinel,
            ModerationModel::Arbiter,
        ]
    }

    pub fn concrete_models() -> Vec<ModerationModel> {
        vec![
            ModerationModel::Observer,
            ModerationModel::Sentinel,
            ModerationModel::Arbiter,
        ]
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, ModerationModel::Auto)
    }

    pub fn to_name(&self) -> &str {
        match self {
            ModerationModel::Auto => "Auto",
            ModerationModel::Observer => "Observer",
            ModerationModel::Sentinel => "Sentinel",
            ModerationModel::Arbiter => "Arbiter",
        }
    }

    pub fn credits_per_byte(&self) -> i64 {
        match self {
            ModerationModel::Auto => {
                panic!("Auto must be resolved before calculating credits")
            }
            ModerationModel::Observer => 1,
            ModerationModel::Sentinel => 3,
            ModerationModel::Arbiter => 9,
        }
    }

    /// Resolve Auto to a concrete model based on remaining credits and allowed models.
    ///
    /// Uses weighted allocation: given n models sorted best-to-worst with weights n, n-1, ..., 1,
    /// the threshold to use model at rank r is: sum(1..r) / total_weight * max_credits.
    pub fn resolve_auto(
        remaining_credits: i64,
        max_credits: i64,
        allowed_models: &[ModerationModel],
    ) -> ModerationModel {
        let mut concrete: Vec<ModerationModel> = allowed_models
            .iter()
            .filter(|m| !m.is_auto())
            .cloned()
            .collect();

        if concrete.is_empty() {
            return ModerationModel::Observer;
        }

        // Sort by credits_per_byte descending (best/most expensive first)
        concrete.sort_by(|a, b| b.credits_per_byte().cmp(&a.credits_per_byte()));

        let n = concrete.len() as i64;
        let total_weight = n * (n + 1) / 2;

        // For each model at rank r (0-indexed), threshold = sum(1..=r) / total_weight * max_credits
        // sum(1..=r) for r=0 is 0, r=1 is 1, r=2 is 3, etc.
        let mut cumulative = 0i64;
        for (i, model) in concrete.iter().enumerate() {
            // cumulative = sum of weights of all models ranked worse (higher index)
            // threshold = cumulative / total_weight * max_credits
            let threshold = if max_credits > 0 {
                cumulative * max_credits / total_weight
            } else {
                0
            };

            if remaining_credits > threshold {
                return model.clone();
            }

            // Add weight of this rank: weight = n - i
            cumulative += n - i as i64;
        }

        // Fallback to cheapest
        concrete.last().cloned().unwrap_or(ModerationModel::Observer)
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
