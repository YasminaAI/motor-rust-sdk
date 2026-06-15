# Reference
## Quotes
<details><summary><code>client.quotes.<a href="/src/api/resources/quotes/client.rs">show_quote</a>(id: i64) -> Result&lt;QuoteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.quotes.show_quote(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `i64` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.quotes.<a href="/src/api/resources/quotes/client.rs">delete_quote</a>(id: i64) -> Result&lt;DeleteQuoteRequestsIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.quotes.delete_quote(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `i64` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.quotes.<a href="/src/api/resources/quotes/client.rs">list_quotes</a>() -> Result&lt;GetQuoteRequestsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.quotes.list_quotes(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.quotes.<a href="/src/api/resources/quotes/client.rs">request_quotes</a>(request: PostQuoteRequestsRequest) -> Result&lt;QuoteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

For getting prices with benefits. 
The Quote IDs can be used later to issue a policy
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .quotes
        .request_quotes(
            &PostQuoteRequestsRequest {
                owner_id: "owner_id".to_string(),
                phone: "phone".to_string(),
                birthdate: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                car_sequence_number: "car_sequence_number".to_string(),
                car_estimated_cost: 1.1,
                email: None,
                is_ownership_transfer: None,
                current_car_owner_id: None,
                car_model_year: None,
                start_date: None,
                drivers: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**owner_id:** `String` — Owner ID must be 10 digits starting with 1, 2, or 7
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Email address must be valid and belongs to the customer
    
</dd>
</dl>

<dl>
<dd>

**phone:** `String` — Phone number must start with 05 and be 10 digits
    
</dd>
</dl>

<dl>
<dd>

**birthdate:** `String` — Birthdate in YYYY-MM-DD format
    
</dd>
</dl>

<dl>
<dd>

**car_sequence_number:** `String` — Car sequence number must be 8 or 9 digits
    
</dd>
</dl>

<dl>
<dd>

**is_ownership_transfer:** `Option<bool>` — Indicates if the ownership is being transferred
    
</dd>
</dl>

<dl>
<dd>

**current_car_owner_id:** `Option<String>` — Required if is_ownership_transfer is true; 10 digits starting with 1,2,7
    
</dd>
</dl>

<dl>
<dd>

**car_estimated_cost:** `f64` — Estimated cost of the car
    
</dd>
</dl>

<dl>
<dd>

**car_model_year:** `Option<i64>` — Car model year between 1950 and next year
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Desired policy start date in YYYY-MM-DD. Must be between tomorrow and 28 days from today (inclusive). The platform validates this range server-side.
    
</dd>
</dl>

<dl>
<dd>

**drivers:** `Option<Vec<PostQuoteRequestsRequestDriversItem>>` — List of drivers for the vehicle. When provided, the sum of all driving_percentage values must equal 100, and the owner must be included among the drivers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Policies
<details><summary><code>client.policies.<a href="/src/api/resources/policies/client.rs">show_policy</a>(car_policy: i64) -> Result&lt;Policy, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Show a specific policy
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.policies.show_policy(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**car_policy:** `i64` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.policies.<a href="/src/api/resources/policies/client.rs">list_policies</a>(quote_request_id: Option&lt;Option&lt;i64&gt;&gt;, quote_price_id: Option&lt;Option&lt;String&gt;&gt;, provider_policy_id: Option&lt;Option&lt;i64&gt;&gt;, car_sequence_number: Option&lt;Option&lt;String&gt;&gt;, new_owner_id: Option&lt;Option&lt;String&gt;&gt;, previous_owner_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;i64&gt;&gt;, min_price: Option&lt;Option&lt;f64&gt;&gt;, max_price: Option&lt;Option&lt;f64&gt;&gt;, per_page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;Vec&lt;Policy&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Listing requested policies
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .policies
        .list_policies(
            &ListPoliciesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**quote_request_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**quote_price_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**provider_policy_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**car_sequence_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**new_owner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**previous_owner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**min_price:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**max_price:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**per_page:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.policies.<a href="/src/api/resources/policies/client.rs">issue_policy</a>(request: PostPoliciesRequest) -> Result&lt;Policy, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

For issuing a new policy
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .policies
        .issue_policy(
            &PostPoliciesRequest {
                quote_request_id: 123,
                quote_reference_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
                quote_price_id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
                benefits: None,
                extra_fields: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**quote_request_id:** `i64` — ID of the car quote request
    
</dd>
</dl>

<dl>
<dd>

**quote_reference_id:** `String` — Unique identifier for the quote reference ID (coming from POST /quote-requests)
    
</dd>
</dl>

<dl>
<dd>

**quote_price_id:** `String` — Unique identifier for the quote price ID that exists inside a quote item (coming from POST /quote-requests)
    
</dd>
</dl>

<dl>
<dd>

**benefits:** `Option<Vec<String>>` — List of benefit UUIDs
    
</dd>
</dl>

<dl>
<dd>

**extra_fields:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Optional free-form object with additional fields. Total JSON-encoded size must not exceed 255 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## OtPs
<details><summary><code>client.ot_ps.<a href="/src/api/resources/ot_ps/client.rs">request_otp_for_quote_verification</a>(request: PostQuoteOtpRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint sends a one-time password (OTP) to the provided email and phone number for quote verification. It should be called before creating a quote request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ot_ps
        .request_otp_for_quote_verification(
            &PostQuoteOtpRequest {
                email: "someone@example.com".to_string(),
                phone: "0501234567".to_string(),
                owner_id: "1012345678".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email address of the car owner
    
</dd>
</dl>

<dl>
<dd>

**phone:** `String` — Phone number starting with 05 and containing 10 digits
    
</dd>
</dl>

<dl>
<dd>

**owner_id:** `String` — National ID or Iqama ID of the car owner (10 digits)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ot_ps.<a href="/src/api/resources/ot_ps/client.rs">request_otp_for_issuing_policy</a>(request: PostIssueOtpRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint sends a one-time password (OTP). It should be called before issuing a policy.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use yasminaai_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ot_ps
        .request_otp_for_issuing_policy(
            &PostIssueOtpRequest {
                email: "someone@example.com".to_string(),
                phone: "0501234567".to_string(),
                owner_id: "1012345678".to_string(),
                quote_request_id: 123,
                quote_reference_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
                quote_price_id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email address of the car owner
    
</dd>
</dl>

<dl>
<dd>

**phone:** `String` — Phone number starting with 05 and containing 10 digits
    
</dd>
</dl>

<dl>
<dd>

**owner_id:** `String` — National ID or Iqama ID of the car owner (10 digits)
    
</dd>
</dl>

<dl>
<dd>

**quote_request_id:** `i64` — ID of the car quote request
    
</dd>
</dl>

<dl>
<dd>

**quote_reference_id:** `String` — Unique identifier for the quote reference ID (coming from POST /quote-requests)
    
</dd>
</dl>

<dl>
<dd>

**quote_price_id:** `String` — Unique identifier for the quote price ID that exists inside a quote item (coming from POST /quote-requests)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

