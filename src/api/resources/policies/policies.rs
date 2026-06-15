use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PoliciesClient {
    pub http_client: HttpClient,
}

impl PoliciesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Show a specific policy
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn show_policy(
        &self,
        car_policy: i64,
        options: Option<RequestOptions>,
    ) -> Result<Policy, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("policies/{}", car_policy),
                None,
                None,
                options,
            )
            .await
    }

    /// Listing requested policies
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_policies(
        &self,
        request: &ListPoliciesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Policy>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "policies",
                None,
                QueryBuilder::new()
                    .int("quote_request_id", request.quote_request_id.clone())
                    .string("quote_price_id", request.quote_price_id.clone())
                    .int("provider_policy_id", request.provider_policy_id.clone())
                    .string("car_sequence_number", request.car_sequence_number.clone())
                    .string("new_owner_id", request.new_owner_id.clone())
                    .string("previous_owner_id", request.previous_owner_id.clone())
                    .int("status", request.status.clone())
                    .float("min_price", request.min_price.clone())
                    .float("max_price", request.max_price.clone())
                    .int("per_page", request.per_page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// For issuing a new policy
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn issue_policy(
        &self,
        request: &PostPoliciesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Policy, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "policies",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
