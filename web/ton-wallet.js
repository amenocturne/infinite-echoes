// TON Wallet integration
let tonConnectUI;
let contractInfo = {
    pieceVersion: null,
    vaultVersion: null,
    feeParams: null,
    securityParams: null
};

// Initialize TON wallet bridge for communication with WASM
window.tonBridge = {
    // Get contract information
    getContractInfo: () => {
        return contractInfo;
    },
    // Will be called from WASM to check connection status
    isWalletConnected: () => {
        return tonConnectUI && tonConnectUI.connected;
    },

    // Will be called from WASM to get user address
    getUserAddress: () => {
        if (tonConnectUI && tonConnectUI.account) {
            return tonConnectUI.account.address;
        }
        return null;
    },

    // Called from WASM to save audio graph (to be implemented later)
    saveAudioGraph: async (audioGraphData) => {
        console.log("Save audio graph requested:", audioGraphData);
        // Will implement TON contract interaction in next phase
        return false;
    },

    // Called from WASM to load audio graph (to be implemented later)
    loadAudioGraph: async (nftAddress) => {
        console.log("Load audio graph requested:", nftAddress);
        // Will implement TON contract interaction in next phase
        return null;
    }
};

// Function to fetch and update contract information
async function fetchContractInfo() {
    try {
        // Check if tonApi is available
        if (!window.tonApi) {
            console.warn("TON API not available yet, will retry in 1 second");
            setTimeout(fetchContractInfo, 1000);
            return null;
        }
        
        // Fetch contract information
        const pieceVersion = await window.tonApi.getPieceVersion();
        const vaultVersion = await window.tonApi.getVaultVersion();
        const feeParams = await window.tonApi.getFeeParams();
        const securityParams = await window.tonApi.getSecurityParams();
        
        // Update the global contract info
        contractInfo = {
            pieceVersion,
            vaultVersion,
            feeParams,
            securityParams
        };
        
        // Update the UI
        updateContractInfoDisplay();
        
        console.log("Contract info loaded:", contractInfo);
        return contractInfo;
    } catch (error) {
        console.error("Error fetching contract info:", error);
        // Retry after a delay if there was an error
        setTimeout(fetchContractInfo, 3000);
        return null;
    }
}

// Update the contract info display
function updateContractInfoDisplay() {
    const contractInfoElement = document.getElementById('contract-info');
    if (!contractInfoElement) return;
    
    if (contractInfo && contractInfo.pieceVersion !== null) {
        contractInfoElement.innerHTML = `
            <div>Piece Version: ${contractInfo.pieceVersion}</div>
            ${contractInfo.vaultVersion !== null ? `<div>Vault Version: ${contractInfo.vaultVersion}</div>` : ''}
        `;
    } else {
        contractInfoElement.innerHTML = '<div>Loading contract info...</div>';
    }
}

// Wait for the document to be fully loaded
document.addEventListener('DOMContentLoaded', () => {
    console.log("DOM loaded, setting up TON wallet UI");
    
    // Initialize contract info display
    updateContractInfoDisplay();
    
    // Fetch contract information
    fetchContractInfo();

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
    const loadTonConnectUI = () => {
        return new Promise((resolve, reject) => {
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

    // Initialize TON Connect UI
    const initializeTonConnect = async () => {
        try {
            // Load the script if not already loaded
            await loadTonConnectUI();

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
            tonConnectUI.onStatusChange(wallet => {
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
    };

    // Initialize TON Connect
    initializeTonConnect();
});

// Update the wallet status display
function updateWalletStatus(connected) {
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
