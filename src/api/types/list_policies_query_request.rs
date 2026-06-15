pub use crate::prelude::*;

/// Query parameters for listPolicies
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPoliciesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_request_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_price_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_policy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_sequence_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

impl ListPoliciesQueryRequest {
    pub fn builder() -> ListPoliciesQueryRequestBuilder {
        <ListPoliciesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPoliciesQueryRequestBuilder {
    quote_request_id: Option<i64>,
    quote_price_id: Option<String>,
    provider_policy_id: Option<i64>,
    car_sequence_number: Option<String>,
    new_owner_id: Option<String>,
    previous_owner_id: Option<String>,
    status: Option<i64>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    per_page: Option<i64>,
}

impl ListPoliciesQueryRequestBuilder {
    pub fn quote_request_id(mut self, value: i64) -> Self {
        self.quote_request_id = Some(value);
        self
    }

    pub fn quote_price_id(mut self, value: impl Into<String>) -> Self {
        self.quote_price_id = Some(value.into());
        self
    }

    pub fn provider_policy_id(mut self, value: i64) -> Self {
        self.provider_policy_id = Some(value);
        self
    }

    pub fn car_sequence_number(mut self, value: impl Into<String>) -> Self {
        self.car_sequence_number = Some(value.into());
        self
    }

    pub fn new_owner_id(mut self, value: impl Into<String>) -> Self {
        self.new_owner_id = Some(value.into());
        self
    }

    pub fn previous_owner_id(mut self, value: impl Into<String>) -> Self {
        self.previous_owner_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn min_price(mut self, value: f64) -> Self {
        self.min_price = Some(value);
        self
    }

    pub fn max_price(mut self, value: f64) -> Self {
        self.max_price = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPoliciesQueryRequest`].
    pub fn build(self) -> Result<ListPoliciesQueryRequest, BuildError> {
        Ok(ListPoliciesQueryRequest {
            quote_request_id: self.quote_request_id,
            quote_price_id: self.quote_price_id,
            provider_policy_id: self.provider_policy_id,
            car_sequence_number: self.car_sequence_number,
            new_owner_id: self.new_owner_id,
            previous_owner_id: self.previous_owner_id,
            status: self.status,
            min_price: self.min_price,
            max_price: self.max_price,
            per_page: self.per_page,
        })
    }
}
