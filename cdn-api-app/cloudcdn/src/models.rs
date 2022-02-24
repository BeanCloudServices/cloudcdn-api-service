#![allow(unused_qualifications)]
#![allow(missing_docs)]

#[cfg(any(feature = "client", feature = "server"))]
use crate::header;
use crate::models;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DistributionUpsert {
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl DistributionUpsert {
    pub fn new(idempotency_key: String) -> DistributionUpsert {
        DistributionUpsert {
            idempotency_key: idempotency_key,
            id: None,
        }
    }
}

/// Converts the DistributionUpsert value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DistributionUpsert {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("idempotencyKey".to_string());
        params.push(self.idempotency_key.to_string());

        // Skipping id in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DistributionUpsert value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DistributionUpsert {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub idempotency_key: Vec<String>,
            pub id: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DistributionUpsert".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "idempotencyKey" => intermediate_rep.idempotency_key.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "id" => intermediate_rep.id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DistributionUpsert".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DistributionUpsert {
            idempotency_key: intermediate_rep
                .idempotency_key
                .into_iter()
                .next()
                .ok_or("idempotencyKey missing in DistributionUpsert".to_string())?,
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DistributionUpsert> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DistributionUpsert>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DistributionUpsert>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DistributionUpsert - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DistributionUpsert>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DistributionUpsert as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DistributionUpsert - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DistributionUpsertAllOf {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl DistributionUpsertAllOf {
    pub fn new() -> DistributionUpsertAllOf {
        DistributionUpsertAllOf { id: None }
    }
}

/// Converts the DistributionUpsertAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DistributionUpsertAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping id in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DistributionUpsertAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DistributionUpsertAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DistributionUpsertAllOf".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(
                        <uuid::Uuid as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DistributionUpsertAllOf".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DistributionUpsertAllOf {
            id: intermediate_rep.id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DistributionUpsertAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DistributionUpsertAllOf>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DistributionUpsertAllOf>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DistributionUpsertAllOf - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DistributionUpsertAllOf>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DistributionUpsertAllOf as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DistributionUpsertAllOf - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DistributionUpsertRequest {
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,
}

impl DistributionUpsertRequest {
    pub fn new(idempotency_key: String) -> DistributionUpsertRequest {
        DistributionUpsertRequest {
            idempotency_key: idempotency_key,
        }
    }
}

/// Converts the DistributionUpsertRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DistributionUpsertRequest {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("idempotencyKey".to_string());
        params.push(self.idempotency_key.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DistributionUpsertRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DistributionUpsertRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub idempotency_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DistributionUpsertRequest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "idempotencyKey" => intermediate_rep.idempotency_key.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DistributionUpsertRequest".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DistributionUpsertRequest {
            idempotency_key: intermediate_rep
                .idempotency_key
                .into_iter()
                .next()
                .ok_or("idempotencyKey missing in DistributionUpsertRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DistributionUpsertRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DistributionUpsertRequest>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DistributionUpsertRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DistributionUpsertRequest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DistributionUpsertRequest>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DistributionUpsertRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DistributionUpsertRequest - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
