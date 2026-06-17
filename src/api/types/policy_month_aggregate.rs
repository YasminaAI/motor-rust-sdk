pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PolicyMonthAggregate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_price: Option<f64>,
}

impl PolicyMonthAggregate {
    pub fn builder() -> PolicyMonthAggregateBuilder {
        <PolicyMonthAggregateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PolicyMonthAggregateBuilder {
    count: Option<i64>,
    total_price: Option<f64>,
}

impl PolicyMonthAggregateBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn total_price(mut self, value: f64) -> Self {
        self.total_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PolicyMonthAggregate`].
    pub fn build(self) -> Result<PolicyMonthAggregate, BuildError> {
        Ok(PolicyMonthAggregate {
            count: self.count,
            total_price: self.total_price,
        })
    }
}
