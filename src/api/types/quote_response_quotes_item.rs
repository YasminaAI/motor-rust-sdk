pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuoteResponseQuotesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// An array representing each price. This will have the premium and the deductible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<QuotePrice>>,
    /// An array representing the different benefits offered by the company. Some of them are free and comes with the insurance, some are paid and optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benefits: Option<Vec<Benefit>>,
}

impl QuoteResponseQuotesItem {
    pub fn builder() -> QuoteResponseQuotesItemBuilder {
        <QuoteResponseQuotesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteResponseQuotesItemBuilder {
    company_name: Option<String>,
    prices: Option<Vec<QuotePrice>>,
    benefits: Option<Vec<Benefit>>,
}

impl QuoteResponseQuotesItemBuilder {
    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn prices(mut self, value: Vec<QuotePrice>) -> Self {
        self.prices = Some(value);
        self
    }

    pub fn benefits(mut self, value: Vec<Benefit>) -> Self {
        self.benefits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteResponseQuotesItem`].
    pub fn build(self) -> Result<QuoteResponseQuotesItem, BuildError> {
        Ok(QuoteResponseQuotesItem {
            company_name: self.company_name,
            prices: self.prices,
            benefits: self.benefits,
        })
    }
}
