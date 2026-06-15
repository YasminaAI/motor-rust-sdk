pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostQuoteRequestsRequest {
    /// Owner ID must be 10 digits starting with 1, 2, or 7
    #[serde(default)]
    pub owner_id: String,
    /// Email address must be valid and belongs to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number must start with 05 and be 10 digits
    #[serde(default)]
    pub phone: String,
    /// Birthdate in YYYY-MM-DD format
    #[serde(default)]
    pub birthdate: NaiveDate,
    /// Car sequence number must be 8 or 9 digits
    #[serde(default)]
    pub car_sequence_number: String,
    /// Indicates if the ownership is being transferred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ownership_transfer: Option<bool>,
    /// Required if is_ownership_transfer is true; 10 digits starting with 1,2,7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_car_owner_id: Option<String>,
    /// Estimated cost of the car
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub car_estimated_cost: f64,
    /// Car model year between 1950 and next year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_model_year: Option<i64>,
    /// Desired policy start date in YYYY-MM-DD. Must be between tomorrow and 28 days from today (inclusive). The platform validates this range server-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// List of drivers for the vehicle. When provided, the sum of all driving_percentage values must equal 100, and the owner must be included among the drivers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers: Option<Vec<PostQuoteRequestsRequestDriversItem>>,
}

impl PostQuoteRequestsRequest {
    pub fn builder() -> PostQuoteRequestsRequestBuilder {
        <PostQuoteRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostQuoteRequestsRequestBuilder {
    owner_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    birthdate: Option<NaiveDate>,
    car_sequence_number: Option<String>,
    is_ownership_transfer: Option<bool>,
    current_car_owner_id: Option<String>,
    car_estimated_cost: Option<f64>,
    car_model_year: Option<i64>,
    start_date: Option<NaiveDate>,
    drivers: Option<Vec<PostQuoteRequestsRequestDriversItem>>,
}

impl PostQuoteRequestsRequestBuilder {
    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    pub fn car_sequence_number(mut self, value: impl Into<String>) -> Self {
        self.car_sequence_number = Some(value.into());
        self
    }

    pub fn is_ownership_transfer(mut self, value: bool) -> Self {
        self.is_ownership_transfer = Some(value);
        self
    }

    pub fn current_car_owner_id(mut self, value: impl Into<String>) -> Self {
        self.current_car_owner_id = Some(value.into());
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

    pub fn drivers(mut self, value: Vec<PostQuoteRequestsRequestDriversItem>) -> Self {
        self.drivers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostQuoteRequestsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owner_id`](PostQuoteRequestsRequestBuilder::owner_id)
    /// - [`phone`](PostQuoteRequestsRequestBuilder::phone)
    /// - [`birthdate`](PostQuoteRequestsRequestBuilder::birthdate)
    /// - [`car_sequence_number`](PostQuoteRequestsRequestBuilder::car_sequence_number)
    /// - [`car_estimated_cost`](PostQuoteRequestsRequestBuilder::car_estimated_cost)
    pub fn build(self) -> Result<PostQuoteRequestsRequest, BuildError> {
        Ok(PostQuoteRequestsRequest {
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            email: self.email,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            birthdate: self
                .birthdate
                .ok_or_else(|| BuildError::missing_field("birthdate"))?,
            car_sequence_number: self
                .car_sequence_number
                .ok_or_else(|| BuildError::missing_field("car_sequence_number"))?,
            is_ownership_transfer: self.is_ownership_transfer,
            current_car_owner_id: self.current_car_owner_id,
            car_estimated_cost: self
                .car_estimated_cost
                .ok_or_else(|| BuildError::missing_field("car_estimated_cost"))?,
            car_model_year: self.car_model_year,
            start_date: self.start_date,
            drivers: self.drivers,
        })
    }
}
