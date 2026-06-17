pub use crate::prelude::*;

/// Query parameters for listQuotes
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListQuotesQueryRequest {
    /// Inclusive lower bound for quote request creation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<NaiveDate>,
    /// Inclusive upper bound for quote request creation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<NaiveDate>,
    /// Number of quote requests to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    /// When true, includes quote request totals and monthly buckets for the filtered result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_aggregates: Option<bool>,
}

impl ListQuotesQueryRequest {
    pub fn builder() -> ListQuotesQueryRequestBuilder {
        <ListQuotesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListQuotesQueryRequestBuilder {
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
    per_page: Option<i64>,
    include_aggregates: Option<bool>,
}

impl ListQuotesQueryRequestBuilder {
    pub fn date_from(mut self, value: NaiveDate) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: NaiveDate) -> Self {
        self.date_to = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    pub fn include_aggregates(mut self, value: bool) -> Self {
        self.include_aggregates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListQuotesQueryRequest`].
    pub fn build(self) -> Result<ListQuotesQueryRequest, BuildError> {
        Ok(ListQuotesQueryRequest {
            date_from: self.date_from,
            date_to: self.date_to,
            per_page: self.per_page,
            include_aggregates: self.include_aggregates,
        })
    }
}
