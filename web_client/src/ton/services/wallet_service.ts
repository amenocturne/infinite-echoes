import { TonConnectUI, THEME } from '@tonconnect/ui';
import { Address, beginCell, Cell, toNano } from '@ton/core';
import { MANIFEST_URL, REGISTRY_ADDRESS, TRANSACTION_CONFIG } from '../../config/constants';
import { Wallet } from '../../types';
import { BaseService, Initializable } from './base';
import { errorHandler, ErrorCode, TonError } from './error_handler';
import {
  CreatePiece,
  storeCreatePiece,
} from '../../contracts/build/EchoRegistry/EchoRegistry_EchoRegistry';

/**
 * Service for handling wallet connections and transactions
 */
export class WalletService extends BaseService implements Initializable {
  private tonConnectUI: TonConnectUI | null = null;
  private walletStatusListeners: Array<(connected: boolean) => void> = [];

  /**
   * Initializes the wallet service
   */
  async initialize(): Promise<void> {
    try {
      this.tonConnectUI = new TonConnectUI({
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

      // Set up status change listener
      this.tonConnectUI.onStatusChange((wallet: Wallet | null) => {
        const isConnected = wallet !== null;
        this.notifyWalletStatusListeners(isConnected);
      });
    } catch (error) {
      this.logError("initialize", error);
      throw errorHandler.handleError(error, "WalletService.initialize");
    }
  }

  /**
   * Opens the wallet connection modal
   */
  async openModal(): Promise<void> {
    if (!this.tonConnectUI) {
      throw new TonError(
        "TonConnectUI not initialized",
        ErrorCode.WALLET_ERROR
      );
    }

    try {
      await this.tonConnectUI.openModal();
    } catch (error) {
      this.logError("openModal", error);
      throw errorHandler.handleError(error, "WalletService.openModal");
    }
  }

  /**
   * Disconnects the wallet
   */
  async disconnect(): Promise<void> {
    if (!this.tonConnectUI) {
      throw new TonError(
        "TonConnectUI not initialized",
        ErrorCode.WALLET_ERROR
      );
    }

    try {
      await this.tonConnectUI.disconnect();
    } catch (error) {
      this.logError("disconnect", error);
      throw errorHandler.handleError(error, "WalletService.disconnect");
    }
  }

  /**
   * Checks if a wallet is connected
   */
  isConnected(): boolean {
    return !!(this.tonConnectUI && this.tonConnectUI.connected);
  }

  /**
   * Gets the connected wallet address
   */
  getWalletAddress(): string | null {
    if (!this.tonConnectUI || !this.tonConnectUI.account) {
      return null;
    }
    return this.tonConnectUI.account.address;
  }

  /**
   * Formats a wallet address for display
   */
  formatAddress(address: string): string {
    return address.substring(0, 6) + '...' + address.substring(address.length - 4);
  }

  /**
   * Sends a message to the registry contract to create a new piece
   * @param pieceRawData Raw data string for the piece
   * @param remixedFrom Optional address this piece was remixed from
   */
  async createNewPiece(
    pieceRawData: string,
    remixedFrom: Address | null = null,
  ): Promise<boolean> {
    if (!this.tonConnectUI || !this.isConnected()) {
      throw new TonError(
        "Wallet not connected. Please connect your wallet first.",
        ErrorCode.WALLET_ERROR
      );
    }

    try {
      // Construct the Cell from the raw data
      const buffer = Buffer.from(pieceRawData);
      const pieceDataCell = beginCell()
        .storeBuffer(buffer)
        .endCell();

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

      const result = await this.tonConnectUI.sendTransaction(finalTransaction);
      console.log('Transaction sent:', result);
      return true;
    } catch (error) {
      this.logError("createNewPiece", error);
      throw errorHandler.handleError(error, "WalletService.createNewPiece");
    }
  }

  /**
   * Subscribes to wallet status changes
   * @param listener Function to call when wallet status changes
   * @returns Function to unsubscribe
   */
  subscribeToWalletStatus(listener: (connected: boolean) => void): () => void {
    this.walletStatusListeners.push(listener);
    
    // Call listener immediately with current status
    listener(this.isConnected());
    
    // Return unsubscribe function
    return () => {
      this.walletStatusListeners = this.walletStatusListeners.filter(l => l !== listener);
    };
  }

  /**
   * Notifies all listeners of wallet status changes
   */
  private notifyWalletStatusListeners(connected: boolean): void {
    this.walletStatusListeners.forEach(listener => listener(connected));
  }
}

// Export a singleton instance for use across the application
export const walletService = new WalletService();
