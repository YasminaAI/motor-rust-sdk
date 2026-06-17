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
    /// * `date_from` - Inclusive lower bound for the policy date. For issued policies (`status=1`), this filters by `uploaded_at` (the provider policy issue timestamp) and falls back to `created_at` when `uploaded_at` is unavailable. For other statuses, this filters by `created_at`.
    /// * `date_to` - Inclusive upper bound for the policy date. For issued policies (`status=1`), this filters by `uploaded_at` (the provider policy issue timestamp) and falls back to `created_at` when `uploaded_at` is unavailable. For other statuses, this filters by `created_at`.
    /// * `include_aggregates` - When true, includes policy totals, total price, and monthly buckets for the filtered result set.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_policies(
        &self,
        request: &ListPoliciesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedPolicyResponse, ApiError> {
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
                    .date("date_from", request.date_from.clone())
                    .date("date_to", request.date_to.clone())
                    .bool("include_aggregates", request.include_aggregates.clone())
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
