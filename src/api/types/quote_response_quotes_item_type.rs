pub use crate::prelude::*;

/// Normalised insurance category used to group and filter quotes. Always one of `TPL`, `TPL +`, or `Comprehensive`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuoteResponseQuotesItemType {
    Tpl,
    Comprehensive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuoteResponseQuotesItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tpl => serializer.serialize_str("TPL"),
            Self::Comprehensive => serializer.serialize_str("Comprehensive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuoteResponseQuotesItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TPL" => Ok(Self::Tpl),
            "Comprehensive" => Ok(Self::Comprehensive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuoteResponseQuotesItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tpl => write!(f, "TPL"),
            Self::Comprehensive => write!(f, "Comprehensive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
