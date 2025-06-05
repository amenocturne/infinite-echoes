import { contractInfo, fetchContractInfo, updateContractInfoDisplay } from './ton_api';
import { TonConnectUI, THEME } from '@tonconnect/ui';
import { Address, beginCell, Cell, toNano } from '@ton/core';
import { CreatePiece, storeCreatePiece } from '../contracts/build/EchoRegistry/EchoRegistry_EchoRegistry';

interface Wallet {
  account: {
    address: string;
  };
}

let tonConnectUI: TonConnectUI;

export async function initializeTonConnectUI(): Promise<void> {
  const container = document.getElementById('ton-connect-ui');
  if (!container) {
    console.error('Container #ton-connect-ui not found!');
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

  try {
    console.log('Initializing TON Connect UI');

    tonConnectUI = new TonConnectUI({
      manifestUrl: 'https://infinite-echoes.app/tonconnect-manifest.json',
      uiPreferences: {
        theme: THEME.DARK,
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

// Function to send a message to the registry contract to create a new piece
// Accepts raw data (string) and constructs the Cell internally.
export async function createNewPiece(pieceRawData: string, remixedFrom: Address | null = null): Promise<boolean> {
  if (!tonConnectUI || !tonConnectUI.connected) {
    console.error('Wallet not connected. Please connect your wallet first.');
    alert('Wallet not connected. Please connect your wallet first.');
    return false;
  }

  const REGISTRY_ADDRESS = 'kQAlmlGXp3ElXKyeLSEXnhacMq117VjqOuzN9r8AJPVEpchv'; // This should ideally come from a config or ton_api

  try {
    // Construct the Cell from the raw data
    const pieceDataCell = beginCell().storeStringTail(pieceRawData).endCell();

    // Use the generated message wrapper to construct the payload
    const createPieceMessage: CreatePiece = {
      $$type: 'CreatePiece',
      pieceData: pieceDataCell,
      remixedFrom: remixedFrom,
    };

    // Corrected payload generation using storeCreatePiece
    const payloadCell = beginCell();
    storeCreatePiece(createPieceMessage)(payloadCell); // Use the store function from the generated wrapper
    const finalPayload = payloadCell.endCell().toBoc().toString('base64');

    const finalTransaction = {
      validUntil: Math.floor(Date.now() / 1000) + 360, // 6 minutes
      messages: [
        {
          address: REGISTRY_ADDRESS,
          amount: toNano('0.1').toString(), // Example amount, adjust as needed for fees
          payload: finalPayload,
        },
      ],
    };


    console.log('Sending transaction to create new piece:', finalTransaction);
    const result = await tonConnectUI.sendTransaction(finalTransaction);
    console.log('Transaction sent:', result);
    alert('Transaction sent successfully! Check your wallet for status.');
    return true;
  } catch (error) {
    console.error('Error sending transaction to create new piece:', error);
    alert(`Failed to send transaction: ${error instanceof Error ? error.message : String(error)}`);
    return false;
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
  createNewPiece: createNewPiece, // Expose the new function
};

export function setupTonWalletIntegration(): void {
  console.log('DOM loaded, setting up TON wallet UI');

  updateContractInfoDisplay();

  fetchContractInfo();

  initializeTonConnectUI();
}
