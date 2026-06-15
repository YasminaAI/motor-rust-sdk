pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteQuoteRequestsIdResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DeleteQuoteRequestsIdResponse {
    pub fn builder() -> DeleteQuoteRequestsIdResponseBuilder {
        <DeleteQuoteRequestsIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteQuoteRequestsIdResponseBuilder {
    message: Option<String>,
}

impl DeleteQuoteRequestsIdResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteQuoteRequestsIdResponse`].
    pub fn build(self) -> Result<DeleteQuoteRequestsIdResponse, BuildError> {
        Ok(DeleteQuoteRequestsIdResponse {
            message: self.message,
        })
    }
}
