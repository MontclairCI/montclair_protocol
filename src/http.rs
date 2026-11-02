use muzanci_config::config::DebugClientConfig;
use muzanci_config::config::DebugSessionId;
use serde::Deserialize;
use serde::Serialize;

use crate::message::AccountId;
use crate::message::UserId;

/// The HTTP Upgrade header value for the MuzanCI transport protocol.
pub const MUZANCI_TRANSPORT_V1: &str = "muzanci-transport/v1";

/// The HTTP header for the runner ID, sent by the server upon successful upgrade to the MuzanCI transport protocol.
/// The value must be a valid [`RunnerId`].
pub const MUZANCI_RUNNER_ID_HEADER: &str = "X-MuzanCI-Runner-ID";

/// The HTTP response body for the RegisterRunner endpoint.
///
/// Sent by the server when the HTTP connection fails to upgrade to the MuzanCI transport protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRunnerResponse {
    pub error: String,
}

/// The HTTP response body for the BeginDebugResolver and BeginDebugClient endpoint.
///
/// Sent by the server when the HTTP connection fails to upgrade to the MuzanCI transport protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugClientResponse {
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliListAccountsResponse {
    pub accounts: Vec<CliAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliAccount {
    pub account_id: AccountId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliGetUserResponse {
    pub user_id_opt: Option<UserId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliCreateDebugSessionRequest {
    pub capacity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliCreateDebugSessionResponse {
    pub debug_session_id: DebugSessionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliGetDebugClientConfigResponse {
    pub debug_client_config_opt: Option<DebugClientConfig>,
}
