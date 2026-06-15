pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostQuoteRequestsRequestDriversItem {
    /// Driver's national ID. Must be 10 digits starting with 1, 2, or 7.
    #[serde(default)]
    pub owner_id: String,
    /// Driver's birthdate in YYYY-MM-DD format.
    #[serde(default)]
    pub birthdate: NaiveDate,
    /// Percentage of driving for this driver. Valid values are 25, 50, 75, or 100. The sum of all drivers' percentages must equal 100.
    #[serde(default)]
    pub driving_percentage: i64,
}

impl PostQuoteRequestsRequestDriversItem {
    pub fn builder() -> PostQuoteRequestsRequestDriversItemBuilder {
        <PostQuoteRequestsRequestDriversItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostQuoteRequestsRequestDriversItemBuilder {
    owner_id: Option<String>,
    birthdate: Option<NaiveDate>,
    driving_percentage: Option<i64>,
}

impl PostQuoteRequestsRequestDriversItemBuilder {
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

    /// Consumes the builder and constructs a [`PostQuoteRequestsRequestDriversItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owner_id`](PostQuoteRequestsRequestDriversItemBuilder::owner_id)
    /// - [`birthdate`](PostQuoteRequestsRequestDriversItemBuilder::birthdate)
    /// - [`driving_percentage`](PostQuoteRequestsRequestDriversItemBuilder::driving_percentage)
    pub fn build(self) -> Result<PostQuoteRequestsRequestDriversItem, BuildError> {
        Ok(PostQuoteRequestsRequestDriversItem {
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            birthdate: self
                .birthdate
                .ok_or_else(|| BuildError::missing_field("birthdate"))?,
            driving_percentage: self
                .driving_percentage
                .ok_or_else(|| BuildError::missing_field("driving_percentage"))?,
        })
    }
}
