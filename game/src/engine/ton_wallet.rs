use miniquad::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self, Promise};

use crate::engine::contract_info::{ContractInfo, FeeParams, SecurityParams};

// JavaScript bindings for TON wallet functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn isWalletConnected() -> bool;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn registryAddress() -> Option<String>;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getUserAddress() -> Option<String>;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getContractInfo() -> JsValue; // Returns the ContractInfo object

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getUserVaultAddress() -> Option<String>;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getPieceAddresses() -> Option<Box<[JsValue]>>;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getPieceData() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn refreshVaultAddress() -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn saveAudioGraph(data: &str) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn loadAudioGraph(address: &str) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn createNewPiece(pieceRawData: &str, remixedFrom: Option<String>) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn setPendingPieceData(pieceRawData: &str, remixedFrom: Option<String>);

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getPendingPieceData() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn clearPendingPieceData();
}

/// Parse the JsValue into a ContractInfo struct
fn parse_contract_info(js_value: &JsValue) -> ContractInfo {
    if js_value.is_undefined() || js_value.is_null() {
        info!("Contract info is null or undefined");
        return ContractInfo::default();
    }

    let json_str = js_sys::JSON::stringify(js_value)
        .ok()
        .and_then(|v| v.as_string());

    if let Some(json_str) = json_str {
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&json_str) {
            let mut contract_info = ContractInfo::default();
            if let Some(obj) = json_value.as_object() {
                if let Some(addr) = obj.get("userVaultAddress").and_then(|v| v.as_str()) {
                    contract_info.user_vault_address = Some(addr.to_string());
                }
                if let Some(count) = obj.get("pieceCount").and_then(|v| v.as_u64()) {
                    contract_info.piece_count = Some(count as u32);
                }

                // Extract fee params
                if let Some(fee_params) = obj.get("feeParams").and_then(|v| v.as_object()) {
                    let deploy_value = fee_params.get("deployValue").and_then(|v| v.as_u64()).unwrap_or(0);
                    let message_value = fee_params.get("messageValue").and_then(|v| v.as_u64()).unwrap_or(0);

                    contract_info.fee_params = Some(FeeParams {
                        deploy_value,
                        message_value,
                    });
                }

                // Extract security params
                if let Some(security_params) = obj.get("securityParams").and_then(|v| v.as_object()) {
                    let min_action_fee = security_params.get("minActionFee").and_then(|v| v.as_u64()).unwrap_or(0);
                    let cool_down_seconds = security_params.get("coolDownSeconds").and_then(|v| v.as_u64()).unwrap_or(0);

                    contract_info.security_params = Some(SecurityParams {
                        min_action_fee,
                        cool_down_seconds,
                    });
                }

                // Extract piece addresses if not null
                if let Some(addresses) = obj.get("pieceAddresses") {
                    if !addresses.is_null() {
                        if let Some(addr_array) = addresses.as_array() {
                            for addr in addr_array {
                                if let Some(addr_str) = addr.as_str() {
                                    contract_info.piece_addresses.push(addr_str.to_string());
                                }
                            }
                        }
                    }
                }

                // Extract piece data if not null
                if let Some(piece_data) = obj.get("pieceData") {
                    if !piece_data.is_null() {
                        if let Some(data_obj) = piece_data.as_object() {
                            for (key, value) in data_obj {
                                let val = if value.is_null() {
                                    None
                                } else {
                                    value.as_str().map(|s| s.to_string())
                                };
                                contract_info.piece_data.insert(key.clone(), val);
                            }
                        }
                    }
                }
                return contract_info;
            }

            // If we couldn't parse it manually, try the automatic way
            match serde_json::from_str::<ContractInfo>(&json_str) {
                Ok(contract_info) => {
                    info!("Successfully parsed contract info with serde");
                    contract_info
                },
                Err(err) => {
                    info!("Error parsing contract info with serde_json: {:?}", err);
                    ContractInfo::default()
                }
            }
        } else {
            info!("Failed to parse JSON string as Value");
            ContractInfo::default()
        }
    } else {
        info!("Failed to stringify JS value");
        ContractInfo::default()
    }
}

/// TON wallet integration for the game
pub enum TransactionState {
    Idle,
    InProgress,
    Completed(bool), // bool indicates success or failure
    Failed(String),
}

pub struct TonWallet {
    connected: bool,
    user_address: Option<String>,
    user_vault_address: Option<String>,
    registry_address: Option<String>,
    transaction_state: TransactionState,
    transaction_data: Option<(String, Option<String>)>, // (piece_data, remixed_from)
    contract_info: ContractInfo,
}

impl TonWallet {
    pub fn new() -> Self {
        // Check initial connection status
        let connected = isWalletConnected();
        let user_address = if connected { getUserAddress() } else { None };
        let user_vault_address = if connected {
            getUserVaultAddress()
        } else {
            None
        };
        let registry_address = registryAddress();
        let js_contract_info = getContractInfo();
        let contract_info = parse_contract_info(&js_contract_info);

        Self {
            connected,
            user_address,
            user_vault_address,
            registry_address,
            transaction_state: TransactionState::Idle,
            transaction_data: None,
            contract_info,
        }
    }

    /// Update wallet connection status and contract info
    pub fn update(&mut self) {
        self.connected = isWalletConnected();
        self.user_address = if self.connected {
            getUserAddress()
        } else {
            None
        };
        // user_vault_address is updated by the JS side when fetchContractInfo is called
        self.user_vault_address = if self.connected {
            getUserVaultAddress()
        } else {
            None
        };

        // Only try to get contract info if connected
        if self.connected {
            let js_contract_info = getContractInfo();
            self.contract_info = parse_contract_info(&js_contract_info);
        } else {
            // Use default contract info when not connected
            self.contract_info = ContractInfo::default();
        }
    }

    /// Check if wallet is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get user's wallet address
    pub fn user_address(&self) -> Option<&str> {
        self.user_address.as_deref()
    }
    pub fn contract_info(&self) -> &ContractInfo {
        &self.contract_info
    }

    /// Get user's vault address
    pub fn user_vault_address(&self) -> Option<&str> {
        self.user_vault_address.as_deref()
    }

    /// Get user's vault address
    pub fn registry_address(&self) -> Option<&str> {
        self.registry_address.as_deref()
    }

    /// Format user address for display (truncated)
    pub fn formatted_address(&self) -> String {
        if let Some(addr) = &self.user_address {
            if addr.len() > 10 {
                format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
            } else {
                addr.clone()
            }
        } else {
            "Not connected".to_string()
        }
    }

    /// Get piece addresses from the TON bridge
    pub fn get_piece_addresses(&self) -> Vec<String> {
        if !self.connected {
            return Vec::new();
        }

        if let Some(addresses) = getPieceAddresses() {
            let mut result = Vec::new();
            for i in 0..addresses.len() {
                if let Some(js_value) = addresses.get(i) {
                    if let Some(addr) = js_value.as_string() {
                        result.push(addr);
                    }
                }
            }
            result
        } else {
            Vec::new()
        }
    }

    /// Get piece data from the TON bridge
    pub fn get_piece_data(&self) -> JsValue {
        getPieceData()
    }

    /// Refresh the user's vault address
    pub async fn refresh_vault_address(&mut self) -> Result<JsValue, JsValue> {
        JsFuture::from(refreshVaultAddress()).await
    }

    /// Save audio graph data to the blockchain
    pub fn save_audio_graph(&self, data: &str) -> Promise {
        // TODO:
        if !self.connected {
            // Create a resolved promise with false value
            return Promise::resolve(&JsValue::from_bool(false));
        }
        saveAudioGraph(data)
    }

    /// Load audio graph data from the blockchain
    pub fn load_audio_graph(&self, address: &str) -> Promise {
        // TODO:
        if !self.connected {
            // Create a resolved promise with null value
            return Promise::resolve(&JsValue::null());
        }
        loadAudioGraph(address)
    }

    /// Check if a transaction is currently in progress
    pub fn is_transaction_in_progress(&self) -> bool {
        matches!(self.transaction_state, TransactionState::InProgress)
    }

    /// Get the current transaction state
    pub fn transaction_state(&self) -> &TransactionState {
        &self.transaction_state
    }

    /// Set pending piece data for the frontend to process
    pub fn set_pending_piece_data(&mut self, piece_raw_data: &str, remixed_from: Option<&str>) {
        if !self.connected {
            info!("Cannot set pending piece data: wallet not connected");
            return;
        }

        let remixed_from_js = remixed_from.map(|s| s.to_string());

        // Set the pending piece data in the JavaScript bridge
        setPendingPieceData(piece_raw_data, remixed_from_js);

        info!("Pending piece data set for frontend processing");
    }

    /// Clear any pending piece data
    pub fn clear_pending_piece_data(&self) {
        clearPendingPieceData();
        info!("Pending piece data cleared");
    }

    /// Legacy method for backward compatibility
    pub async fn create_new_piece(
        &self,
        piece_raw_data: &str,
        remixed_from: Option<&str>,
    ) -> Result<JsValue, JsValue> {
        if !self.connected {
            return Result::Err(JsValue::from_str("Wallet not connected"));
        }

        let remixed_from_js = remixed_from.map(|s| s.to_string());

        info!("Starting transaction from Rust...");

        // Create a Promise that won't resolve until the user completes the transaction
        let promise = createNewPiece(piece_raw_data, remixed_from_js);

        // This will block until the Promise resolves or rejects
        info!("Waiting for transaction to complete...");
        let result = JsFuture::from(promise).await;

        // Log the result for debugging
        info!("Transaction result received");
        info!("{:?}", result);

        result
    }
}
