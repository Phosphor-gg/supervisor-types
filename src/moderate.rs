use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_labels: Option<Vec<ModerationLabel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_context: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_labels: Option<Vec<ModerationLabel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewritten_text: Option<String>,
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
    #[serde(rename = "profanity", alias = "P")]
    P,
    #[serde(rename = "toxicity", alias = "T")]
    T,
    #[serde(rename = "harassment", alias = "H")]
    H,
    #[serde(rename = "hate", alias = "HR")]
    HR,
    #[serde(rename = "insult", alias = "I")]
    I,
    #[serde(rename = "sexual", alias = "S")]
    S,
    #[serde(rename = "sexual/minors", alias = "SU")]
    SU,
    #[serde(rename = "sexual/explicit", alias = "S2")]
    S2,
    #[serde(rename = "sensitive", alias = "SE")]
    SE,
    #[serde(rename = "violence", alias = "V")]
    V,
    #[serde(rename = "self-harm", alias = "SH")]
    SH,
    #[serde(rename = "medical", alias = "M")]
    M,
    #[serde(rename = "spam", alias = "SP")]
    SP,
    #[serde(rename = "promotional", alias = "PM")]
    PM,
    #[serde(rename = "scam", alias = "SI")]
    SI,
    #[serde(rename = "illegal", alias = "IL")]
    IL,
    #[serde(rename = "personal-data", alias = "PD")]
    PD,
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
            ModerationLabel::P => write!(f, "P"),
            ModerationLabel::T => write!(f, "T"),
            ModerationLabel::H => write!(f, "H"),
            ModerationLabel::HR => write!(f, "HR"),
            ModerationLabel::I => write!(f, "I"),
            ModerationLabel::S => write!(f, "S"),
            ModerationLabel::SU => write!(f, "SU"),
            ModerationLabel::S2 => write!(f, "S2"),
            ModerationLabel::SE => write!(f, "SE"),
            ModerationLabel::V => write!(f, "V"),
            ModerationLabel::SH => write!(f, "SH"),
            ModerationLabel::M => write!(f, "M"),
            ModerationLabel::SP => write!(f, "SP"),
            ModerationLabel::PM => write!(f, "PM"),
            ModerationLabel::SI => write!(f, "SI"),
            ModerationLabel::IL => write!(f, "IL"),
            ModerationLabel::PD => write!(f, "PD"),
        }
    }
}

impl FromStr for ModerationLabel {
    type Err = ();

    fn from_str(input: &str) -> Result<ModerationLabel, Self::Err> {
        match input {
            "P" => Ok(ModerationLabel::P),
            "T" => Ok(ModerationLabel::T),
            "H" => Ok(ModerationLabel::H),
            "HR" => Ok(ModerationLabel::HR),
            "I" => Ok(ModerationLabel::I),
            "S" => Ok(ModerationLabel::S),
            "SU" => Ok(ModerationLabel::SU),
            "S2" => Ok(ModerationLabel::S2),
            "SE" => Ok(ModerationLabel::SE),
            "V" => Ok(ModerationLabel::V),
            "SH" => Ok(ModerationLabel::SH),
            "M" => Ok(ModerationLabel::M),
            "SP" => Ok(ModerationLabel::SP),
            "PM" => Ok(ModerationLabel::PM),
            "SI" => Ok(ModerationLabel::SI),
            "IL" => Ok(ModerationLabel::IL),
            "PD" => Ok(ModerationLabel::PD),
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

    pub fn version(&self) -> &str {
        match self {
            ModerationModel::Auto => "2.0",
            ModerationModel::Observer => "2.0",
            ModerationModel::Sentinel => "2.0",
            ModerationModel::Arbiter => "2.0",
        }
    }

    pub fn base_name(&self) -> &str {
        match self {
            ModerationModel::Auto => "Auto",
            ModerationModel::Observer => "Observer",
            ModerationModel::Sentinel => "Sentinel",
            ModerationModel::Arbiter => "Arbiter",
        }
    }

    pub fn to_name(&self) -> String {
        format!("{} {}", self.base_name(), self.version())
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameCheckRequest {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameCheckResponse {
    pub flagged: bool,
    pub score: f64,
}

impl ModerationLabel {
    pub fn to_name(&self) -> &str {
        match self {
            ModerationLabel::P => "Profanity",
            ModerationLabel::T => "Toxicity",
            ModerationLabel::H => "Harassment",
            ModerationLabel::HR => "Hate/Racism",
            ModerationLabel::I => "Insult",
            ModerationLabel::S => "Sexual",
            ModerationLabel::SU => "Sexual (Severe/Minors)",
            ModerationLabel::S2 => "Sexual (Explicit)",
            ModerationLabel::SE => "Sensitive Content",
            ModerationLabel::V => "Violence",
            ModerationLabel::SH => "Self-Harm",
            ModerationLabel::M => "Medical/Injury",
            ModerationLabel::SP => "Spam",
            ModerationLabel::PM => "Promotional",
            ModerationLabel::SI => "Scam/Incoherent",
            ModerationLabel::IL => "Illegal Activity",
            ModerationLabel::PD => "Personal Data",
        }
    }

    pub fn to_short_name(&self) -> &str {
        match self {
            ModerationLabel::P => "P",
            ModerationLabel::T => "T",
            ModerationLabel::H => "H",
            ModerationLabel::HR => "HR",
            ModerationLabel::I => "I",
            ModerationLabel::S => "S",
            ModerationLabel::SU => "SU",
            ModerationLabel::S2 => "S2",
            ModerationLabel::SE => "SE",
            ModerationLabel::V => "V",
            ModerationLabel::SH => "SH",
            ModerationLabel::M => "M",
            ModerationLabel::SP => "SP",
            ModerationLabel::PM => "PM",
            ModerationLabel::SI => "SI",
            ModerationLabel::IL => "IL",
            ModerationLabel::PD => "PD",
        }
    }

    pub fn all_labels() -> Vec<ModerationLabel> {
        vec![
            ModerationLabel::P,
            ModerationLabel::T,
            ModerationLabel::H,
            ModerationLabel::HR,
            ModerationLabel::I,
            ModerationLabel::S,
            ModerationLabel::SU,
            ModerationLabel::S2,
            ModerationLabel::SE,
            ModerationLabel::V,
            ModerationLabel::SH,
            ModerationLabel::M,
            ModerationLabel::SP,
            ModerationLabel::PM,
            ModerationLabel::SI,
            ModerationLabel::IL,
            ModerationLabel::PD,
        ]
    }
}
