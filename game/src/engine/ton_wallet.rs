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
    fn saveAudioGraph(data: &str) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "tonBridge"])]
    fn loadAudioGraph(address: &str) -> Promise;
}

/// TON wallet integration for the game
pub struct TonWallet {
    connected: bool,
    user_address: Option<String>,
}

impl TonWallet {
    pub fn new() -> Self {
        // Check initial connection status
        let connected = isWalletConnected();
        let user_address = if connected { getUserAddress() } else { None };

        Self {
            connected,
            user_address,
        }
    }

    /// Update wallet connection status
    pub fn update(&mut self) {
        self.connected = isWalletConnected();
        self.user_address = if self.connected { getUserAddress() } else { None };
    }

    /// Check if wallet is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get user's wallet address
    pub fn user_address(&self) -> Option<&str> {
        self.user_address.as_deref()
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
}
