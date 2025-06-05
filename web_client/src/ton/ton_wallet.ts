import { contractInfo, fetchContractInfo, updateContractInfoDisplay } from './ton_api';
import { TonConnectUI, THEME } from '@tonconnect/ui';
import { Address, beginCell, Cell, toNano } from '@ton/core';
import {
  CreatePiece,
  storeCreatePiece,
} from '../contracts/build/EchoRegistry/EchoRegistry_EchoRegistry';
import { Wallet } from '../types';
import { MANIFEST_URL, REGISTRY_ADDRESS, TRANSACTION_CONFIG } from '../config/constants';

let tonConnectUI: TonConnectUI;

/**
 * Creates a custom wallet connect button
 */
function createConnectButton(): HTMLButtonElement {
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

  return customButton;
}

/**
 * Initializes the TON Connect UI
 */
export async function initializeTonConnectUI(): Promise<void> {
  const container = document.getElementById('ton-connect-ui');
  if (!container) {
    console.error('Container #ton-connect-ui not found!');
    return;
  }

  const customButton = createConnectButton();
  container.innerHTML = '';
  container.appendChild(customButton);

  try {
    console.log('Initializing TON Connect UI');

    tonConnectUI = new TonConnectUI({
      manifestUrl: MANIFEST_URL,
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

/**
 * Updates the wallet connection status in the UI
 */
function updateWalletStatus(connected: boolean): void {
  const walletStatus = document.getElementById('wallet-status');
  if (!walletStatus) return;

  if (connected && tonConnectUI && tonConnectUI.account) {
    const address = tonConnectUI.account.address;
    const formattedAddress = formatAddress(address);
    walletStatus.textContent = `Connected: ${formattedAddress}`;
    walletStatus.classList.add('connected');
  } else {
    walletStatus.textContent = 'Not connected';
    walletStatus.classList.remove('connected');
  }
}

/**
 * Formats a wallet address for display
 */
function formatAddress(address: string): string {
  return address.substring(0, 6) + '...' + address.substring(address.length - 4);
}

/**
 * Sends a message to the registry contract to create a new piece
 * @param pieceRawData - Raw data string for the piece
 * @param remixedFrom - Optional address this piece was remixed from
 */
export async function createNewPiece(
  pieceRawData: string,
  remixedFrom: Address | null = null,
): Promise<boolean> {
  if (!tonConnectUI || !tonConnectUI.connected) {
    console.error('Wallet not connected. Please connect your wallet first.');
    alert('Wallet not connected. Please connect your wallet first.');
    return false;
  }

  try {
    // Construct the Cell from the raw data
    const pieceDataCell = beginCell().storeStringTail(pieceRawData).endCell();

    // Use the generated message wrapper to construct the payload
    const createPieceMessage: CreatePiece = {
      $$type: 'CreatePiece',
      pieceData: pieceDataCell,
      remixedFrom: remixedFrom,
    };

    // Generate payload using storeCreatePiece
    const payloadCell = beginCell();
    storeCreatePiece(createPieceMessage)(payloadCell);
    const finalPayload = payloadCell.endCell().toBoc().toString('base64');

    const finalTransaction = {
      validUntil: Math.floor(Date.now() / 1000) + TRANSACTION_CONFIG.VALID_SECONDS,
      messages: [
        {
          address: REGISTRY_ADDRESS,
          amount: toNano(TRANSACTION_CONFIG.DEFAULT_AMOUNT).toString(),
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

/**
 * Bridge object for communication between Rust WASM and JavaScript
 */
(window as any).tonBridge = {
  getContractInfo: () => contractInfo,

  isWalletConnected: (): boolean => !!(tonConnectUI && tonConnectUI.connected),

  getUserAddress: (): string | null => tonConnectUI?.account?.address || null,

  saveAudioGraph: async (audioGraphData: string): Promise<boolean> => {
    console.log('Save audio graph requested:', audioGraphData);
    return false;
  },

  loadAudioGraph: async (nftAddress: string): Promise<string | null> => {
    console.log('Load audio graph requested:', nftAddress);
    return null;
  },

  createNewPiece,
};

/**
 * Sets up the TON wallet integration
 */
export function setupTonWalletIntegration(): void {
  console.log('Setting up TON wallet integration');

  // Initialize in sequence
  updateContractInfoDisplay();
  fetchContractInfo();
  initializeTonConnectUI();
}
