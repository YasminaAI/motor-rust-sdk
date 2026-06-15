pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Benefit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_benefit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Benefit {
    pub fn builder() -> BenefitBuilder {
        <BenefitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BenefitBuilder {
    quote_benefit_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    amount: Option<f64>,
    vat: Option<f64>,
    url: Option<String>,
}

impl BenefitBuilder {
    pub fn quote_benefit_id(mut self, value: impl Into<String>) -> Self {
        self.quote_benefit_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn vat(mut self, value: f64) -> Self {
        self.vat = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Benefit`].
    pub fn build(self) -> Result<Benefit, BuildError> {
        Ok(Benefit {
            quote_benefit_id: self.quote_benefit_id,
            id: self.id,
            name: self.name,
            amount: self.amount,
            vat: self.vat,
            url: self.url,
        })
    }
}
