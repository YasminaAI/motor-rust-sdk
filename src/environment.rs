use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "sandbox")]
    Sandbox,
    #[serde(rename = "production")]
    Production,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Sandbox => "https://sandbox.yasmina.ai/api/v1/car-comp",
            Self::Production => "https://production.yasmina.ai/api/v1/car-comp",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Sandbox
    }
}
