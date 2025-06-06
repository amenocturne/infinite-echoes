use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

// JavaScript bindings for TON wallet functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn isWalletConnected() -> bool;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getUserAddress() -> Option<String>;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getContractInfo() -> JsValue; // Returns the ContractInfo object

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn getUserVaultAddress() -> Option<String>;

    // Removed refreshVaultAddress as it requires wasm-bindgen-futures
    // #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    // fn refreshVaultAddress() -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn saveAudioGraph(data: &str) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn loadAudioGraph(address: &str) -> Promise;
}

/// TON wallet integration for the game
pub struct TonWallet {
    connected: bool,
    user_address: Option<String>,
    user_vault_address: Option<String>,
}

impl TonWallet {
    pub fn new() -> Self {
        // Check initial connection status
        let connected = isWalletConnected();
        let user_address = if connected { getUserAddress() } else { None };
        let user_vault_address = if connected { getUserVaultAddress() } else { None };

        Self {
            connected,
            user_address,
            user_vault_address,
        }
    }

    /// Update wallet connection status and contract info
    pub fn update(&mut self) {
        self.connected = isWalletConnected();
        self.user_address = if self.connected { getUserAddress() } else { None };
        // user_vault_address is updated by the JS side when fetchContractInfo is called
        self.user_vault_address = if self.connected { getUserVaultAddress() } else { None };
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

    /// Format user address for display (truncated)
    pub fn formatted_address(&self) -> String {
        if let Some(addr) = &self.user_address {
            if addr.len() > 10 {
                format!("{}...{}", &addr[..6], &addr[addr.len()-4..])
            } else {
                addr.clone()
            }
        } else {
            "Not connected".to_string()
        }
    }

    // Removed refresh_vault_address async function due to dependency constraint
    // pub async fn refresh_vault_address(&mut self) -> Result<(), JsValue> {
    //     if self.connected && self.user_address.is_some() {
    //         let promise = refreshVaultAddress();
    //         let js_value = wasm_bindgen_futures::JsFuture::from(promise).await?;
    //         self.user_vault_address = js_value.as_string();
    //     }
    //     Ok(())
    // }
}
