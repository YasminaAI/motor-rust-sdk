pub use crate::prelude::*;

/// Returned only when include_aggregates is true.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PolicyAggregates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_price: Option<f64>,
    /// Monthly policy counts and sales totals keyed by YYYY-MM. For issued policies (`status=1`), buckets use `uploaded_at` and fall back to `created_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month: Option<HashMap<String, PolicyMonthAggregate>>,
}

impl PolicyAggregates {
    pub fn builder() -> PolicyAggregatesBuilder {
        <PolicyAggregatesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PolicyAggregatesBuilder {
    total_count: Option<i64>,
    total_price: Option<f64>,
    by_month: Option<HashMap<String, PolicyMonthAggregate>>,
}

impl PolicyAggregatesBuilder {
    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_price(mut self, value: f64) -> Self {
        self.total_price = Some(value);
        self
    }

    pub fn by_month(mut self, value: HashMap<String, PolicyMonthAggregate>) -> Self {
        self.by_month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PolicyAggregates`].
    pub fn build(self) -> Result<PolicyAggregates, BuildError> {
        Ok(PolicyAggregates {
            total_count: self.total_count,
            total_price: self.total_price,
            by_month: self.by_month,
        })
    }
}
