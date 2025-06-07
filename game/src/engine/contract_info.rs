use serde::{Deserialize, Serialize};

use super::ton_wallet::PieceData;

/// Represents the contract information from the TON blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    /// Fee parameters for contract operations
    #[serde(rename = "feeParams")]
    pub fee_params: Option<FeeParams>,

    /// Security parameters for contract operations
    #[serde(rename = "securityParams")]
    pub security_params: Option<SecurityParams>,

    /// User's vault address
    #[serde(rename = "userVaultAddress")]
    pub user_vault_address: Option<String>,

    /// Count of pieces owned by the user
    #[serde(rename = "pieceCount")]
    pub piece_count: Option<u32>,

    /// Addresses of pieces owned by the user
    #[serde(rename = "pieceAddresses", default)]
    pub piece_addresses: Vec<String>,

    /// Data associated with each piece
    #[serde(rename = "pieceData", default)]
    pub piece_data: std::collections::HashMap<String, Option<String>>,

    /// Deserialized cards for each piece
    #[serde(skip)]
    pub piece_data_structs: std::collections::HashMap<String, PieceData>,
}

/// Fee parameters for contract operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeParams {
    /// Value required for deployment
    #[serde(rename = "deployValue")]
    pub deploy_value: u64,

    /// Value required for messages
    #[serde(rename = "messageValue")]
    pub message_value: u64,
}

/// Security parameters for contract operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityParams {
    /// Minimum fee required for actions
    #[serde(rename = "minActionFee")]
    pub min_action_fee: u64,

    /// Cooldown period in seconds
    #[serde(rename = "coolDownSeconds")]
    pub cool_down_seconds: u64,
}

impl Default for ContractInfo {
    fn default() -> Self {
        Self {
            fee_params: None,
            security_params: None,
            user_vault_address: None,
            piece_count: None,
            piece_addresses: Vec::new(),
            piece_data: std::collections::HashMap::new(),
            piece_data_structs: std::collections::HashMap::new(),
        }
    }
}
