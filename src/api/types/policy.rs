pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_policy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_claimed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_document: Option<String>,
}

impl Policy {
    pub fn builder() -> PolicyBuilder {
        <PolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PolicyBuilder {
    id: Option<i64>,
    meta_data: Option<HashMap<String, serde_json::Value>>,
    start_date: Option<String>,
    provider_policy_id: Option<i64>,
    provider_policy: Option<String>,
    order_status: Option<i64>,
    approval_status: Option<i64>,
    end_date: Option<String>,
    is_claimed: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
    client_id: Option<String>,
    canceled_at: Option<String>,
    invoice: Option<String>,
    cancellation_document: Option<String>,
}

impl PolicyBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn meta_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta_data = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn provider_policy_id(mut self, value: i64) -> Self {
        self.provider_policy_id = Some(value);
        self
    }

    pub fn provider_policy(mut self, value: impl Into<String>) -> Self {
        self.provider_policy = Some(value.into());
        self
    }

    pub fn order_status(mut self, value: i64) -> Self {
        self.order_status = Some(value);
        self
    }

    pub fn approval_status(mut self, value: i64) -> Self {
        self.approval_status = Some(value);
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn is_claimed(mut self, value: bool) -> Self {
        self.is_claimed = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn canceled_at(mut self, value: impl Into<String>) -> Self {
        self.canceled_at = Some(value.into());
        self
    }

    pub fn invoice(mut self, value: impl Into<String>) -> Self {
        self.invoice = Some(value.into());
        self
    }

    pub fn cancellation_document(mut self, value: impl Into<String>) -> Self {
        self.cancellation_document = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Policy`].
    pub fn build(self) -> Result<Policy, BuildError> {
        Ok(Policy {
            id: self.id,
            meta_data: self.meta_data,
            start_date: self.start_date,
            provider_policy_id: self.provider_policy_id,
            provider_policy: self.provider_policy,
            order_status: self.order_status,
            approval_status: self.approval_status,
            end_date: self.end_date,
            is_claimed: self.is_claimed,
            created_at: self.created_at,
            updated_at: self.updated_at,
            client_id: self.client_id,
            canceled_at: self.canceled_at,
            invoice: self.invoice,
            cancellation_document: self.cancellation_document,
        })
    }
}
