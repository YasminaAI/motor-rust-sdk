pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuotePrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_price_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deductible: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub subtotal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vat_percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total: Option<f64>,
}

impl QuotePrice {
    pub fn builder() -> QuotePriceBuilder {
        <QuotePriceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuotePriceBuilder {
    quote_price_id: Option<String>,
    deductible: Option<f64>,
    subtotal: Option<f64>,
    vat_percentage: Option<f64>,
    vat: Option<f64>,
    total: Option<f64>,
}

impl QuotePriceBuilder {
    pub fn quote_price_id(mut self, value: impl Into<String>) -> Self {
        self.quote_price_id = Some(value.into());
        self
    }

    pub fn deductible(mut self, value: f64) -> Self {
        self.deductible = Some(value);
        self
    }

    pub fn subtotal(mut self, value: f64) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn vat_percentage(mut self, value: f64) -> Self {
        self.vat_percentage = Some(value);
        self
    }

    pub fn vat(mut self, value: f64) -> Self {
        self.vat = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuotePrice`].
    pub fn build(self) -> Result<QuotePrice, BuildError> {
        Ok(QuotePrice {
            quote_price_id: self.quote_price_id,
            deductible: self.deductible,
            subtotal: self.subtotal,
            vat_percentage: self.vat_percentage,
            vat: self.vat,
            total: self.total,
        })
    }
}
