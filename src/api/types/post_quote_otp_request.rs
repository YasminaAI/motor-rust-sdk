pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostQuoteOtpRequest {
    /// Email address of the car owner
    #[serde(default)]
    pub email: String,
    /// Phone number starting with 05 and containing 10 digits
    #[serde(default)]
    pub phone: String,
    /// National ID or Iqama ID of the car owner (10 digits)
    #[serde(default)]
    pub owner_id: String,
}

impl PostQuoteOtpRequest {
    pub fn builder() -> PostQuoteOtpRequestBuilder {
        <PostQuoteOtpRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostQuoteOtpRequestBuilder {
    email: Option<String>,
    phone: Option<String>,
    owner_id: Option<String>,
}

impl PostQuoteOtpRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostQuoteOtpRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](PostQuoteOtpRequestBuilder::email)
    /// - [`phone`](PostQuoteOtpRequestBuilder::phone)
    /// - [`owner_id`](PostQuoteOtpRequestBuilder::owner_id)
    pub fn build(self) -> Result<PostQuoteOtpRequest, BuildError> {
        Ok(PostQuoteOtpRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
        })
    }
}
