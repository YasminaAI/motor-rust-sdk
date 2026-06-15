pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnauthorizedErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl UnauthorizedErrorBody {
    pub fn builder() -> UnauthorizedErrorBodyBuilder {
        <UnauthorizedErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnauthorizedErrorBodyBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl UnauthorizedErrorBodyBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnauthorizedErrorBody`].
    pub fn build(self) -> Result<UnauthorizedErrorBody, BuildError> {
        Ok(UnauthorizedErrorBody {
            code: self.code,
            message: self.message,
        })
    }
}
