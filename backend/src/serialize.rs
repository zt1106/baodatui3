use rsocket_rust::prelude::Payload;
use serde_json::Value;

use crate::ext::IntoResult;

/// un-typed request with command
pub struct RawRequest {
    pub data: Value,
    pub command: String,
}

/// un-typed response with data or error
pub struct RawResponse {
    pub data: Option<Value>,
    pub success: bool,
    pub error: Option<String>,
}

fn value_to_payload(value: Value) -> anyhow::Result<Payload> {
    let data = serde_json::to_string(&value)?;
    let payload = Payload::builder().set_data_utf8(&data).build();
    Ok(payload)
}

fn payload_to_value(payload: &Payload) -> anyhow::Result<Value> {
    let data = payload.data_utf8().into_result()?;
    let value = serde_json::from_str(data)?;
    Ok(value)
}

pub fn payload_to_raw_request(payload: &Payload) -> anyhow::Result<RawRequest> {
    let mut value = payload_to_value(payload)?;
    let command = value["command"].as_str().into_result()?.to_string();
    let data = value["data"].take();
    Ok(RawRequest { data, command })
}

pub fn raw_response_to_payload(raw_response: RawResponse) -> anyhow::Result<Payload> {
    let mut value = serde_json::json!({
        "success": raw_response.success,
    });
    if let Some(data) = raw_response.data {
        value["data"] = data;
    }
    if let Some(error) = raw_response.error {
        value["error"] = Value::String(error);
    }
    value_to_payload(value)
}
