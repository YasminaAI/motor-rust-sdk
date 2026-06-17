pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompanyQuote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Arabic name of the insurance company. Use this field instead of `company_name` when rendering Arabic UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name_ar: Option<String>,
    /// Normalised insurance category used to group and filter quotes. Always one of `TPL`, `TPL +`, or `Comprehensive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CompanyQuoteType>,
    /// The insurance type label exactly as the insurance provider intends it to be displayed. While `type` normalises all non-TPL / non-Comprehensive values into `TPL +`, this field preserves the original provider string (e.g. "TPL Plus", "Third Party Plus") and should be shown in the UI wherever the provider's own wording is preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_type_display: Option<String>,
    /// Arabic translation of `insurance_type_display`. Use this field for Arabic UIs. Falls back to the English value for provider-specific types that do not have a translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_type_display_ar: Option<String>,
    /// CDN URL for the insurance company's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_logo_url: Option<String>,
    /// CDN URL for the insurance company's square logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_company_logo_url: Option<String>,
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
    company_name_ar: Option<String>,
    r#type: Option<CompanyQuoteType>,
    insurance_type_display: Option<String>,
    insurance_type_display_ar: Option<String>,
    company_logo_url: Option<String>,
    square_company_logo_url: Option<String>,
    prices: Option<Vec<QuotePrice>>,
    benefits: Option<Vec<Benefit>>,
}

impl CompanyQuoteBuilder {
    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn company_name_ar(mut self, value: impl Into<String>) -> Self {
        self.company_name_ar = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CompanyQuoteType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn insurance_type_display(mut self, value: impl Into<String>) -> Self {
        self.insurance_type_display = Some(value.into());
        self
    }

    pub fn insurance_type_display_ar(mut self, value: impl Into<String>) -> Self {
        self.insurance_type_display_ar = Some(value.into());
        self
    }

    pub fn company_logo_url(mut self, value: impl Into<String>) -> Self {
        self.company_logo_url = Some(value.into());
        self
    }

    pub fn square_company_logo_url(mut self, value: impl Into<String>) -> Self {
        self.square_company_logo_url = Some(value.into());
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
            company_name_ar: self.company_name_ar,
            r#type: self.r#type,
            insurance_type_display: self.insurance_type_display,
            insurance_type_display_ar: self.insurance_type_display_ar,
            company_logo_url: self.company_logo_url,
            square_company_logo_url: self.square_company_logo_url,
            prices: self.prices,
            benefits: self.benefits,
        })
    }
}
