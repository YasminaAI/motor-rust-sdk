pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginationLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl PaginationLink {
    pub fn builder() -> PaginationLinkBuilder {
        <PaginationLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationLinkBuilder {
    url: Option<String>,
    label: Option<String>,
    active: Option<bool>,
}

impl PaginationLinkBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginationLink`].
    pub fn build(self) -> Result<PaginationLink, BuildError> {
        Ok(PaginationLink {
            url: self.url,
            label: self.label,
            active: self.active,
        })
    }
}
