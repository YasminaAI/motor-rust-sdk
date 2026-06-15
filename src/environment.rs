use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "default")]
    Default,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Default => "https://staging.yasmina.ai/api/v1/car-comp",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Default
    }
}
