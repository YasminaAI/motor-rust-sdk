//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Quotes**
//! - **Policies**
//! - **OtPs**

use crate::{ApiError, ClientConfig};

pub mod ot_ps;
pub mod policies;
pub mod quotes;
pub struct ApiClient {
    pub config: ClientConfig,
    pub quotes: QuotesClient,
    pub policies: PoliciesClient,
    pub ot_ps: OtPsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            quotes: QuotesClient::new(config.clone())?,
            policies: PoliciesClient::new(config.clone())?,
            ot_ps: OtPsClient::new(config.clone())?,
        })
    }
}

pub use ot_ps::OtPsClient;
pub use policies::PoliciesClient;
pub use quotes::QuotesClient;
