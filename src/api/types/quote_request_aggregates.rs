pub use crate::prelude::*;

/// Returned only when include_aggregates is true.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuoteRequestAggregates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// Monthly quote request counts keyed by YYYY-MM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month: Option<HashMap<String, i64>>,
}

impl QuoteRequestAggregates {
    pub fn builder() -> QuoteRequestAggregatesBuilder {
        <QuoteRequestAggregatesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteRequestAggregatesBuilder {
    total_count: Option<i64>,
    by_month: Option<HashMap<String, i64>>,
}

impl QuoteRequestAggregatesBuilder {
    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn by_month(mut self, value: HashMap<String, i64>) -> Self {
        self.by_month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteRequestAggregates`].
    pub fn build(self) -> Result<QuoteRequestAggregates, BuildError> {
        Ok(QuoteRequestAggregates {
            total_count: self.total_count,
            by_month: self.by_month,
        })
    }
}
