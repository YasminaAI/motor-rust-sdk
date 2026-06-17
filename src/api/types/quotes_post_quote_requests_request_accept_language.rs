pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostQuoteRequestsRequestAcceptLanguage {
    #[serde(rename = "ar")]
    Ar,
}
impl fmt::Display for PostQuoteRequestsRequestAcceptLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ar => "ar",
        };
        write!(f, "{}", s)
    }
}
