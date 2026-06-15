use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct QuotesClient {
    pub http_client: HttpClient,
}

impl QuotesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn show_quote(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("quote-requests/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn delete_quote(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeleteQuoteRequestsIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("quote-requests/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn list_quotes(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetQuoteRequestsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "quote-requests", None, None, options)
            .await
    }

    /// For getting prices with benefits.
    /// The Quote IDs can be used later to issue a policy
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn request_quotes(
        &self,
        request: &PostQuoteRequestsRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "quote-requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
