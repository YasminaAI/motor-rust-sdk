pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedQuoteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<QuoteResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<PaginationLink>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregates: Option<QuoteRequestAggregates>,
}

impl PaginatedQuoteResponse {
    pub fn builder() -> PaginatedQuoteResponseBuilder {
        <PaginatedQuoteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedQuoteResponseBuilder {
    current_page: Option<i64>,
    data: Option<Vec<QuoteResponse>>,
    first_page_url: Option<String>,
    from: Option<i64>,
    last_page: Option<i64>,
    last_page_url: Option<String>,
    links: Option<Vec<PaginationLink>>,
    next_page_url: Option<String>,
    path: Option<String>,
    per_page: Option<i64>,
    prev_page_url: Option<String>,
    to: Option<i64>,
    total: Option<i64>,
    aggregates: Option<QuoteRequestAggregates>,
}

impl PaginatedQuoteResponseBuilder {
    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<QuoteResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn first_page_url(mut self, value: impl Into<String>) -> Self {
        self.first_page_url = Some(value.into());
        self
    }

    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn last_page(mut self, value: i64) -> Self {
        self.last_page = Some(value);
        self
    }

    pub fn last_page_url(mut self, value: impl Into<String>) -> Self {
        self.last_page_url = Some(value.into());
        self
    }

    pub fn links(mut self, value: Vec<PaginationLink>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn next_page_url(mut self, value: impl Into<String>) -> Self {
        self.next_page_url = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    pub fn prev_page_url(mut self, value: impl Into<String>) -> Self {
        self.prev_page_url = Some(value.into());
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn aggregates(mut self, value: QuoteRequestAggregates) -> Self {
        self.aggregates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedQuoteResponse`].
    pub fn build(self) -> Result<PaginatedQuoteResponse, BuildError> {
        Ok(PaginatedQuoteResponse {
            current_page: self.current_page,
            data: self.data,
            first_page_url: self.first_page_url,
            from: self.from,
            last_page: self.last_page,
            last_page_url: self.last_page_url,
            links: self.links,
            next_page_url: self.next_page_url,
            path: self.path,
            per_page: self.per_page,
            prev_page_url: self.prev_page_url,
            to: self.to,
            total: self.total,
            aggregates: self.aggregates,
        })
    }
}
