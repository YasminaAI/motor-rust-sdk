pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuoteResponse {
    /// The owner’s national ID or Iqama ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    /// The owner's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The owner's birthdate. Hijri for Saudi nationals, Gregorian for others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<NaiveDate>,
    /// The car sequence number from 9 digits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_sequence_number: Option<i64>,
    /// Custom car number for newly imported cars (present when `custom_number` was used in the request)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_number: Option<String>,
    /// Whether it was a car transfer or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ownership_transfer: Option<bool>,
    /// The estimated cost of the car
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub car_estimated_cost: Option<f64>,
    /// The car model year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_model_year: Option<i64>,
    /// Requested policy start date in YYYY-MM-DD. Returned if provided in the quote request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// List of drivers associated with this quote request. Returned if drivers were provided in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers: Option<Vec<QuoteResponseDriversItem>>,
    /// An array representing each insurance company quote. Each item has the company name, the prices, and the benefits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<Vec<QuoteResponseQuotesItem>>,
    /// Your own client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// In case of an update on this quote, this date will change
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// When was the quote requested
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Yasmina ID for the quote. You can use it to delete items or showing it again to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl QuoteResponse {
    pub fn builder() -> QuoteResponseBuilder {
        <QuoteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteResponseBuilder {
    owner_id: Option<i64>,
    phone: Option<String>,
    birthdate: Option<NaiveDate>,
    car_sequence_number: Option<i64>,
    custom_number: Option<String>,
    is_ownership_transfer: Option<bool>,
    car_estimated_cost: Option<f64>,
    car_model_year: Option<i64>,
    start_date: Option<NaiveDate>,
    drivers: Option<Vec<QuoteResponseDriversItem>>,
    quotes: Option<Vec<QuoteResponseQuotesItem>>,
    client_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<i64>,
}

impl QuoteResponseBuilder {
    pub fn owner_id(mut self, value: i64) -> Self {
        self.owner_id = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn birthdate(mut self, value: NaiveDate) -> Self {
        self.birthdate = Some(value);
        self
    }

    pub fn car_sequence_number(mut self, value: i64) -> Self {
        self.car_sequence_number = Some(value);
        self
    }

    pub fn custom_number(mut self, value: impl Into<String>) -> Self {
        self.custom_number = Some(value.into());
        self
    }

    pub fn is_ownership_transfer(mut self, value: bool) -> Self {
        self.is_ownership_transfer = Some(value);
        self
    }

    pub fn car_estimated_cost(mut self, value: f64) -> Self {
        self.car_estimated_cost = Some(value);
        self
    }

    pub fn car_model_year(mut self, value: i64) -> Self {
        self.car_model_year = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn drivers(mut self, value: Vec<QuoteResponseDriversItem>) -> Self {
        self.drivers = Some(value);
        self
    }

    pub fn quotes(mut self, value: Vec<QuoteResponseQuotesItem>) -> Self {
        self.quotes = Some(value);
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteResponse`].
    pub fn build(self) -> Result<QuoteResponse, BuildError> {
        Ok(QuoteResponse {
            owner_id: self.owner_id,
            phone: self.phone,
            birthdate: self.birthdate,
            car_sequence_number: self.car_sequence_number,
            custom_number: self.custom_number,
            is_ownership_transfer: self.is_ownership_transfer,
            car_estimated_cost: self.car_estimated_cost,
            car_model_year: self.car_model_year,
            start_date: self.start_date,
            drivers: self.drivers,
            quotes: self.quotes,
            client_id: self.client_id,
            updated_at: self.updated_at,
            created_at: self.created_at,
            id: self.id,
        })
    }
}
