pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteResponseDriversItem {
    /// Driver's national ID (10 digits starting with 1, 2, or 7)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// Driver's birthdate in YYYY-MM-DD format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<NaiveDate>,
    /// Percentage of driving for this driver (25, 50, 75, or 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driving_percentage: Option<i64>,
}

impl QuoteResponseDriversItem {
    pub fn builder() -> QuoteResponseDriversItemBuilder {
        <QuoteResponseDriversItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteResponseDriversItemBuilder {
    owner_id: Option<String>,
    birthdate: Option<NaiveDate>,
    driving_percentage: Option<i64>,
}

impl QuoteResponseDriversItemBuilder {
    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn birthdate(mut self, value: NaiveDate) -> Self {
        self.birthdate = Some(value);
        self
    }

    pub fn driving_percentage(mut self, value: i64) -> Self {
        self.driving_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteResponseDriversItem`].
    pub fn build(self) -> Result<QuoteResponseDriversItem, BuildError> {
        Ok(QuoteResponseDriversItem {
            owner_id: self.owner_id,
            birthdate: self.birthdate,
            driving_percentage: self.driving_percentage,
        })
    }
}
