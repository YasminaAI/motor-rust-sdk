pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostIssueOtpRequest {
    /// Email address of the car owner
    #[serde(default)]
    pub email: String,
    /// Phone number starting with 05 and containing 10 digits
    #[serde(default)]
    pub phone: String,
    /// National ID or Iqama ID of the car owner (10 digits)
    #[serde(default)]
    pub owner_id: String,
    /// ID of the car quote request
    #[serde(default)]
    pub quote_request_id: i64,
    /// Unique identifier for the quote reference ID (coming from POST /quote-requests)
    #[serde(default)]
    pub quote_reference_id: String,
    /// Unique identifier for the quote price ID that exists inside a quote item (coming from POST /quote-requests)
    #[serde(default)]
    pub quote_price_id: String,
}

impl PostIssueOtpRequest {
    pub fn builder() -> PostIssueOtpRequestBuilder {
        <PostIssueOtpRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostIssueOtpRequestBuilder {
    email: Option<String>,
    phone: Option<String>,
    owner_id: Option<String>,
    quote_request_id: Option<i64>,
    quote_reference_id: Option<String>,
    quote_price_id: Option<String>,
}

impl PostIssueOtpRequestBuilder {
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

    pub fn quote_request_id(mut self, value: i64) -> Self {
        self.quote_request_id = Some(value);
        self
    }

    pub fn quote_reference_id(mut self, value: impl Into<String>) -> Self {
        self.quote_reference_id = Some(value.into());
        self
    }

    pub fn quote_price_id(mut self, value: impl Into<String>) -> Self {
        self.quote_price_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostIssueOtpRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](PostIssueOtpRequestBuilder::email)
    /// - [`phone`](PostIssueOtpRequestBuilder::phone)
    /// - [`owner_id`](PostIssueOtpRequestBuilder::owner_id)
    /// - [`quote_request_id`](PostIssueOtpRequestBuilder::quote_request_id)
    /// - [`quote_reference_id`](PostIssueOtpRequestBuilder::quote_reference_id)
    /// - [`quote_price_id`](PostIssueOtpRequestBuilder::quote_price_id)
    pub fn build(self) -> Result<PostIssueOtpRequest, BuildError> {
        Ok(PostIssueOtpRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            quote_request_id: self
                .quote_request_id
                .ok_or_else(|| BuildError::missing_field("quote_request_id"))?,
            quote_reference_id: self
                .quote_reference_id
                .ok_or_else(|| BuildError::missing_field("quote_reference_id"))?,
            quote_price_id: self
                .quote_price_id
                .ok_or_else(|| BuildError::missing_field("quote_price_id"))?,
        })
    }
}
