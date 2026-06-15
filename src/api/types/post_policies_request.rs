pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostPoliciesRequest {
    /// ID of the car quote request
    #[serde(default)]
    pub quote_request_id: i64,
    /// Unique identifier for the quote reference ID (coming from POST /quote-requests)
    #[serde(default)]
    pub quote_reference_id: String,
    /// Unique identifier for the quote price ID that exists inside a quote item (coming from POST /quote-requests)
    #[serde(default)]
    pub quote_price_id: String,
    /// List of benefit UUIDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benefits: Option<Vec<String>>,
    /// Optional free-form object with additional fields. Total JSON-encoded size must not exceed 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<HashMap<String, serde_json::Value>>,
}

impl PostPoliciesRequest {
    pub fn builder() -> PostPoliciesRequestBuilder {
        <PostPoliciesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPoliciesRequestBuilder {
    quote_request_id: Option<i64>,
    quote_reference_id: Option<String>,
    quote_price_id: Option<String>,
    benefits: Option<Vec<String>>,
    extra_fields: Option<HashMap<String, serde_json::Value>>,
}

impl PostPoliciesRequestBuilder {
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

    pub fn benefits(mut self, value: Vec<String>) -> Self {
        self.benefits = Some(value);
        self
    }

    pub fn extra_fields(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extra_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPoliciesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quote_request_id`](PostPoliciesRequestBuilder::quote_request_id)
    /// - [`quote_reference_id`](PostPoliciesRequestBuilder::quote_reference_id)
    /// - [`quote_price_id`](PostPoliciesRequestBuilder::quote_price_id)
    pub fn build(self) -> Result<PostPoliciesRequest, BuildError> {
        Ok(PostPoliciesRequest {
            quote_request_id: self
                .quote_request_id
                .ok_or_else(|| BuildError::missing_field("quote_request_id"))?,
            quote_reference_id: self
                .quote_reference_id
                .ok_or_else(|| BuildError::missing_field("quote_reference_id"))?,
            quote_price_id: self
                .quote_price_id
                .ok_or_else(|| BuildError::missing_field("quote_price_id"))?,
            benefits: self.benefits,
            extra_fields: self.extra_fields,
        })
    }
}
