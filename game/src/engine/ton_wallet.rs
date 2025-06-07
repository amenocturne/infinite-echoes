use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

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
}

/// TON wallet integration for the game
pub struct TonWallet {
    connected: bool,
    user_address: Option<String>,
    user_vault_address: Option<String>,
    registry_address: Option<String>,
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

        Self {
            connected,
            user_address,
            user_vault_address,
            registry_address
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
    }

    /// Check if wallet is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get user's wallet address
    pub fn user_address(&self) -> Option<&str> {
        self.user_address.as_deref()
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
    pub fn save_audio_graph(&self, data: &str) -> Promise { // TODO:
        if !self.connected {
            // Create a resolved promise with false value
            return Promise::resolve(&JsValue::from_bool(false));
        }
        saveAudioGraph(data)
    }

    /// Load audio graph data from the blockchain
    pub fn load_audio_graph(&self, address: &str) -> Promise { // TODO:
        if !self.connected {
            // Create a resolved promise with null value
            return Promise::resolve(&JsValue::null());
        }
        loadAudioGraph(address)
    }

    pub async fn create_new_piece(&self, piece_raw_data: &str, remixed_from: Option<&str>) -> Result<JsValue, JsValue>  {
        if !self.connected {
            return Result::Err(JsValue::from_str("Wallet not connected"));
        }

        let remixed_from_js = remixed_from.map(|s| s.to_string());
        JsFuture::from( createNewPiece(piece_raw_data, remixed_from_js)).await
    }
}
