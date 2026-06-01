use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Entitlement {
    ObserverModel,
    SentinelModel,
    ArbiterModel,
    Context,
    ImplicitLabels,
    CustomBotAppearance,
    PlatformApi,
}

impl Entitlement {
    pub fn lookup_key(&self) -> &'static str {
        match self {
            Self::ObserverModel => "observer-model",
            Self::SentinelModel => "sentinel-model",
            Self::ArbiterModel => "arbiter-model",
            Self::Context => "context",
            Self::ImplicitLabels => "implicit-labels",
            Self::CustomBotAppearance => "custom-bot-appearance",
            Self::PlatformApi => "platform-api",
        }
    }

    pub fn from_lookup_key(key: &str) -> Option<Self> {
        match key {
            "observer-model" => Some(Self::ObserverModel),
            "sentinel-model" => Some(Self::SentinelModel),
            "arbiter-model" => Some(Self::ArbiterModel),
            "context" => Some(Self::Context),
            "implicit-labels" => Some(Self::ImplicitLabels),
            "custom-bot-appearance" => Some(Self::CustomBotAppearance),
            "platform-api" => Some(Self::PlatformApi),
            _ => None,
        }
    }
}
