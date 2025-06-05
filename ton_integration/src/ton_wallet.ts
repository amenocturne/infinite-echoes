import { contractInfo, fetchContractInfo, updateContractInfoDisplay } from "./ton_api";

declare const TON_CONNECT_UI: any; // Declare TON_CONNECT_UI global

// Define a basic Wallet type based on tonconnect-ui's typical structure
interface Wallet {
    account: {
        address: string;
        // Add other properties if needed, e.g., chain, publicKey, etc.
    };
    // Add other wallet properties if needed
}

let tonConnectUI: any;

// Function to initialize TON Connect UI
export async function initializeTonConnectUI(): Promise<void> {
    // Create a container for the TON Connect button
    const container = document.getElementById('ton-connect-ui');
    if (!container) {
        console.error("Container #ton-connect-ui not found");
        return;
    }

    // Create a custom button that will trigger the wallet connection
    const customButton = document.createElement('button');
    customButton.id = 'connect-wallet-btn';
    customButton.textContent = 'Connect TON Wallet';
    customButton.style.backgroundColor = '#0088cc';
    customButton.style.color = 'white';
    customButton.style.border = 'none';
    customButton.style.padding = '8px 16px';
    customButton.style.borderRadius = '4px';
    customButton.style.cursor = 'pointer';
    customButton.style.fontWeight = 'bold';
    customButton.style.width = '100%';

    // Add the button to the container
    container.innerHTML = ''; // Clear any existing content
    container.appendChild(customButton);

    // Load TON Connect UI script dynamically
    const loadTonConnectUIScript = () => {
        return new Promise<void>((resolve, reject) => {
            if (typeof TON_CONNECT_UI !== 'undefined') {
                resolve();
                return;
            }

            const script = document.createElement('script');
            script.src = 'https://unpkg.com/@tonconnect/ui@latest/dist/tonconnect-ui.min.js';
            script.async = true;

            script.onload = () => {
                console.log("TON Connect UI script loaded successfully");
                resolve();
            };

            script.onerror = () => {
                console.error("Failed to load TON Connect UI script");
                reject(new Error("Failed to load TON Connect UI script"));
            };

            document.head.appendChild(script);
        });
    };

    try {
        // Load the script if not already loaded
        await loadTonConnectUIScript();

        console.log("Initializing TON Connect UI");

        // Initialize according to the documentation
        tonConnectUI = new TON_CONNECT_UI.TonConnectUI({
            manifestUrl: 'https://infinite-echoes.app/tonconnect-manifest.json',
            uiPreferences: {
                theme: 'DARK'
            },
            // Add Telegram Mini App return URL if in Telegram
            ...(window.Telegram && window.Telegram.WebApp ? {
                uiOptions: {
                    twaReturnUrl: window.location.origin
                }
            } : {})
        });

        console.log("TON Connect UI initialized successfully");

        // Set up click handler for our custom button
        customButton.addEventListener('click', async () => {
            console.log("Connect button clicked");

            try {
                if (tonConnectUI.connected) {
                    // Disconnect if already connected
                    await tonConnectUI.disconnect();
                    updateWalletStatus(false);
                    customButton.textContent = 'Connect TON Wallet';
                } else {
                    // Open connection modal
                    await tonConnectUI.openModal();
                }
            } catch (error) {
                console.error("Error handling wallet connection:", error);
            }
        });

        // Subscribe to connection status changes
        tonConnectUI.onStatusChange((wallet: Wallet | null) => { // <-- FIX IS HERE: Added type annotation
            const isConnected = wallet !== null;
            updateWalletStatus(isConnected);

            // Update button text
            customButton.textContent = isConnected ? 'Disconnect Wallet' : 'Connect TON Wallet';
        });

        // Check if already connected
        if (tonConnectUI.connected) {
            updateWalletStatus(true);
            customButton.textContent = 'Disconnect Wallet';
        }

    } catch (error) {
        console.error("Error initializing TON Connect UI:", error);

        // Fallback for initialization error
        customButton.addEventListener('click', () => {
            console.error("TON Connect initialization failed:", error);
            alert("TON Connect is not available. Please try again later.");
        });
    }
}

// Update the wallet status display
function updateWalletStatus(connected: boolean): void {
    const walletStatus = document.getElementById('wallet-status');

    if (walletStatus) {
        if (connected && tonConnectUI && tonConnectUI.account) {
            const address = tonConnectUI.account.address;
            // Format address for display (truncate middle)
            const formattedAddress = address.substring(0, 6) + '...' +
                                    address.substring(address.length - 4);
            walletStatus.textContent = `Connected: ${formattedAddress}`;
            walletStatus.classList.add('connected');
        } else {
            walletStatus.textContent = 'Not connected';
            walletStatus.classList.remove('connected');
        }
    }
}

// TON wallet bridge for communication with WASM
// This object will be exposed globally for Rust to call
(window as any).tonBridge = {
    // Get contract information
    getContractInfo: () => {
        return contractInfo;
    },
    // Will be called from WASM to check connection status
    isWalletConnected: (): boolean => {
        return tonConnectUI && tonConnectUI.connected;
    },

    // Will be called from WASM to get user address
    getUserAddress: (): string | null => {
        if (tonConnectUI && tonConnectUI.account) {
            return tonConnectUI.account.address;
        }
        return null;
    },

    // Called from WASM to save audio graph (to be implemented later)
    saveAudioGraph: async (audioGraphData: string): Promise<boolean> => {
        console.log("Save audio graph requested:", audioGraphData);
        // Will implement TON contract interaction in next phase
        return false;
    },

    // Called from WASM to load audio graph (to be implemented later)
    loadAudioGraph: async (nftAddress: string): Promise<string | null> => {
        console.log("Load audio graph requested:", nftAddress);
        // Will implement TON contract interaction in next phase
        return null;
    }
};

// Initial setup when DOM is ready
export function setupTonWalletIntegration(): void {
    console.log("DOM loaded, setting up TON wallet UI");

    // Initialize contract info display
    updateContractInfoDisplay();

    // Fetch contract information (and retry if needed)
    fetchContractInfo();

    // Initialize TON Connect UI
    initializeTonConnectUI();
}
