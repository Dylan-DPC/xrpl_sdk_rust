//! https://xrpl.org/account_lines.html

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountLinesRequestPayload {
    pub account: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // ledger_hash: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // ledger_index: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // strict: Option<bool>,
    // TODO: add more parameters!
}

// TODO: consider extracting as a type.

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLine {
    pub account: String,
    pub balance: String,
    pub currency: String,
    pub limit: String,
    pub limit_peer: String,
    pub no_ripple: bool,
    pub quality_in: u64,
    pub quality_out: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccountLinesResponsePayload {
    pub lines: Vec<AccountLine>,
}
