//! Module for types used in the API.
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::AnthropicError;
use crate::DEFAULT_MODEL;

#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "AnthropicError"))]
pub struct CompleteRequest {
    /// The prompt to complete.
    pub prompt: String,
    /// The model to use.
    #[builder(default = "DEFAULT_MODEL.to_string()")]
    pub model: String,
    /// The number of tokens to sample.
    pub max_tokens_to_sample: usize,
    /// The stop sequences to use.
    pub stop_sequences: Option<Vec<String>>,
    /// Whether to incrementally stream the response.
    #[builder(default = "false")]
    pub stream_response: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CompleteResponse {
    pub completion: String,
    pub stop_reason: StopReason,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum StopReason {
    MaxTokens,
    StopSequence,
}
