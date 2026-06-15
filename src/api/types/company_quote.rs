pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompanyQuote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<QuotePrice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benefits: Option<Vec<Benefit>>,
}

impl CompanyQuote {
    pub fn builder() -> CompanyQuoteBuilder {
        <CompanyQuoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyQuoteBuilder {
    company_name: Option<String>,
    prices: Option<Vec<QuotePrice>>,
    benefits: Option<Vec<Benefit>>,
}

impl CompanyQuoteBuilder {
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

    /// Consumes the builder and constructs a [`CompanyQuote`].
    pub fn build(self) -> Result<CompanyQuote, BuildError> {
        Ok(CompanyQuote {
            company_name: self.company_name,
            prices: self.prices,
            benefits: self.benefits,
        })
    }
}
