import { contractInfo, fetchContractInfo, updateContractInfoDisplay } from './ton_api';

declare const TON_CONNECT_UI: any;

interface Wallet {
  account: {
    address: string;
  };
}

let tonConnectUI: any;

export async function initializeTonConnectUI(): Promise<void> {
  const container = document.getElementById('ton-connect-ui');
  if (!container) {
    console.error('Container #ton-connect-ui not found');
    return;
  }

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

  container.innerHTML = '';
  container.appendChild(customButton);

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
        console.log('TON Connect UI script loaded successfully');
        resolve();
      };

      script.onerror = () => {
        console.error('Failed to load TON Connect UI script');
        reject(new Error('Failed to load TON Connect UI script'));
      };

      document.head.appendChild(script);
    });
  };

  try {
    await loadTonConnectUIScript();

    console.log('Initializing TON Connect UI');

    tonConnectUI = new TON_CONNECT_UI.TonConnectUI({
      manifestUrl: 'https://infinite-echoes.app/tonconnect-manifest.json',
      uiPreferences: {
        theme: 'DARK',
      },
      ...(window.Telegram && window.Telegram.WebApp
        ? {
            uiOptions: {
              twaReturnUrl: window.location.origin,
            },
          }
        : {}),
    });

    console.log('TON Connect UI initialized successfully');

    customButton.addEventListener('click', async () => {
      console.log('Connect button clicked');

      try {
        if (tonConnectUI.connected) {
          await tonConnectUI.disconnect();
          updateWalletStatus(false);
          customButton.textContent = 'Connect TON Wallet';
        } else {
          await tonConnectUI.openModal();
        }
      } catch (error) {
        console.error('Error handling wallet connection:', error);
      }
    });

    tonConnectUI.onStatusChange((wallet: Wallet | null) => {
      const isConnected = wallet !== null;
      updateWalletStatus(isConnected);

      customButton.textContent = isConnected ? 'Disconnect Wallet' : 'Connect TON Wallet';
    });

    if (tonConnectUI.connected) {
      updateWalletStatus(true);
      customButton.textContent = 'Disconnect Wallet';
    }
  } catch (error) {
    console.error('Error initializing TON Connect UI:', error);

    customButton.addEventListener('click', () => {
      console.error('TON Connect initialization failed:', error);
      alert('TON Connect is not available. Please try again later.');
    });
  }
}

function updateWalletStatus(connected: boolean): void {
  const walletStatus = document.getElementById('wallet-status');

  if (walletStatus) {
    if (connected && tonConnectUI && tonConnectUI.account) {
      const address = tonConnectUI.account.address;
      const formattedAddress =
        address.substring(0, 6) + '...' + address.substring(address.length - 4);
      walletStatus.textContent = `Connected: ${formattedAddress}`;
      walletStatus.classList.add('connected');
    } else {
      walletStatus.textContent = 'Not connected';
      walletStatus.classList.remove('connected');
    }
  }
}

(window as any).tonBridge = {
  getContractInfo: () => {
    return contractInfo;
  },
  isWalletConnected: (): boolean => {
    return tonConnectUI && tonConnectUI.connected;
  },

  getUserAddress: (): string | null => {
    if (tonConnectUI && tonConnectUI.account) {
      return tonConnectUI.account.address;
    }
    return null;
  },

  saveAudioGraph: async (audioGraphData: string): Promise<boolean> => {
    console.log('Save audio graph requested:', audioGraphData);
    return false;
  },

  loadAudioGraph: async (nftAddress: string): Promise<string | null> => {
    console.log('Load audio graph requested:', nftAddress);
    return null;
  },
};

export function setupTonWalletIntegration(): void {
  console.log('DOM loaded, setting up TON wallet UI');

  updateContractInfoDisplay();

  fetchContractInfo();

  initializeTonConnectUI();
}
