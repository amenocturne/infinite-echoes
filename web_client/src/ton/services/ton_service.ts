import { Address } from '@ton/core';
import { ContractInfo } from '../../types';
import { BaseService, Initializable } from './base';
import { errorHandler } from './error_handler';
import { pieceService } from './piece_service';
import { registryService } from './registry_service';
import { storageService } from './storage_service';
import { tonStateStore } from './state_store';
import { vaultService } from './vault_service';
import { walletService } from './wallet_service';

/**
 * Main service for coordinating TON-related functionality
 */
export class TonService extends BaseService implements Initializable {
  private isLoadingContractInfo = false;
  private pollingTimeoutId: ReturnType<typeof setTimeout> | null = null;
  private currentUserAddress: string | null = null;

  /**
   * Initializes the TON service
   */
  async initialize(): Promise<void> {
    try {
      await walletService.initialize();

      walletService.subscribeToWalletStatus(async (connected) => {
        if (connected) {
          this.currentUserAddress = walletService.getWalletAddress();
          await this.fetchContractInfo();
          this.startPollingForNewPieces();
        } else {
          this.stopPollingForNewPieces();
          if (this.currentUserAddress) {
            storageService.clearPieces(this.currentUserAddress);
            this.currentUserAddress = null;
          }
          tonStateStore.updateState({
            userVaultAddress: null,
            pieceCount: null,
            pieceAddresses: null,
            pieceData: null,
            pieceRemixData: null,
          });
        }
      });

      this.currentUserAddress = walletService.getWalletAddress();
      await this.fetchContractInfo();
      if (walletService.isConnected()) {
        this.startPollingForNewPieces();
      }
    } catch (error) {
      this.logError('initialize', error);
      throw errorHandler.handleError(error, 'TonService.initialize');
    }
  }

  /**
   * Starts polling for new pieces.
   */
  private startPollingForNewPieces(): void {
    this.stopPollingForNewPieces(); // Prevent multiple loops.
    console.log('[TonService] Started polling for new pieces.');
    this.scheduleNextPoll();
  }

  /**
   * Schedules the next poll using a recursive setTimeout for safer async operations.
   */
  private scheduleNextPoll(): void {
    const POLLING_INTERVAL_MS = 20000;
    this.pollingTimeoutId = setTimeout(async () => {
      await this.checkForNewPieces();
      // If polling is still active (i.e., not stopped by disconnect), schedule the next one.
      if (this.pollingTimeoutId !== null) {
        this.scheduleNextPoll();
      }
    }, POLLING_INTERVAL_MS);
  }

  /**
   * Stops the polling for new pieces.
   */
  private stopPollingForNewPieces(): void {
    if (this.pollingTimeoutId) {
      clearTimeout(this.pollingTimeoutId);
      this.pollingTimeoutId = null;
      console.log('[TonService] Stopped polling for new pieces.');
    }
  }

  /**
   * Checks for new pieces in the user's vault and fetches their data.
   */
  private async checkForNewPieces(): Promise<void> {
    const userAddress = this.currentUserAddress;
    if (!userAddress) {
      this.stopPollingForNewPieces();
      return;
    }

    const currentState = tonStateStore.getState();
    const vaultAddress = currentState.userVaultAddress;

    if (!vaultAddress) {
      return; // Not fully initialized yet, will try again on next poll.
    }

    try {
      const latestPieceAddresses = await vaultService.getPieceAddresses(vaultAddress);
      if (!latestPieceAddresses) {
        return;
      }

      const currentAddresses = new Set(currentState.pieceAddresses || []);
      const newAddresses = latestPieceAddresses.filter((addr) => !currentAddresses.has(addr));

      if (newAddresses.length > 0) {
        console.log(`[TonService] Found ${newAddresses.length} new piece(s). Fetching...`);
        // Update the main list of addresses in the state.
        tonStateStore.updateState({ pieceAddresses: latestPieceAddresses });
        // Fetch data only for the new pieces.
        pieceService.fetchAllPieceData(userAddress, newAddresses);
      }
    } catch (error) {
      this.logError('checkForNewPieces', error);
    }
  }

  /**
   * Fetches all contract information from the registry
   */
  async fetchContractInfo(): Promise<ContractInfo | null> {
    if (this.isLoadingContractInfo) {
      return null;
    }

    try {
      this.isLoadingContractInfo = true;
      tonStateStore.setLoading(true);

      const feeParams = await registryService.getFeeParams();
      const securityParams = await registryService.getSecurityParams();

      tonStateStore.updateState({
        feeParams,
        securityParams,
      });

      const userAddress = this.currentUserAddress;
      if (userAddress) {
        // Load from cache first
        const cachedData = storageService.loadPieces(userAddress);
        if (cachedData) {
          tonStateStore.updateState({
            pieceData: cachedData.pieceData,
            pieceRemixData: cachedData.pieceRemixData,
          });
        }

        const vaultAddress = await registryService.getVaultAddress(userAddress);
        tonStateStore.updateState({ userVaultAddress: vaultAddress });

        if (vaultAddress) {
          const pieceCount = await vaultService.getPieceCount(vaultAddress);
          tonStateStore.updateState({ pieceCount });

          const pieceAddresses = await vaultService.getPieceAddresses(vaultAddress);
          tonStateStore.updateState({ pieceAddresses });

          if (pieceAddresses && pieceAddresses.length > 0) {
            const currentState = tonStateStore.getState();
            const currentPieces = currentState.pieceData || {};
            const addressesToFetch = pieceAddresses.filter(
              (addr) => currentPieces[addr] === undefined,
            );
            if (addressesToFetch.length > 0) {
              pieceService.fetchAllPieceData(userAddress, addressesToFetch);
            }
          } else {
            // No pieces in vault, ensure local state and storage are clean.
            tonStateStore.updateState({ pieceData: {}, pieceRemixData: {} });
            storageService.savePieces(userAddress, { pieceData: {}, pieceRemixData: {} });
          }
        }
      }

      return tonStateStore.getState();
    } catch (error) {
      this.logError('fetchContractInfo', error);
      setTimeout(() => this.fetchContractInfo(), 3000);
      return null;
    } finally {
      this.isLoadingContractInfo = false;
      tonStateStore.setLoading(false);
    }
  }

  /**
   * Refreshes the vault address for the current user
   */
  async refreshVaultAddress(): Promise<string | null> {
    const userAddress = walletService.getWalletAddress();
    if (!userAddress) {
      return null;
    }

    try {
      const vaultAddress = await registryService.getVaultAddress(userAddress);
      tonStateStore.updateState({ userVaultAddress: vaultAddress });
      return vaultAddress;
    } catch (error) {
      this.logError('refreshVaultAddress', error);
      return null;
    }
  }

  /**
   * Creates a new piece on the blockchain
   * @param pieceRawData Raw data string for the piece
   * @param remixedFrom Optional address this piece was remixed from
   */
  async createNewPiece(pieceRawData: string, remixedFrom: Address | null = null): Promise<boolean> {
    try {
      const result = await walletService.createNewPiece(pieceRawData, remixedFrom);
      if (result) {
        setTimeout(() => this.fetchContractInfo(), 5000);
      }
      return result;
    } catch (error) {
      this.logError('createNewPiece', error);
      errorHandler.showError(error);
      return false;
    }
  }

  /**
   * Gets the current contract info
   */
  getContractInfo(): ContractInfo {
    return tonStateStore.getState();
  }

  /**
   * Checks if a wallet is connected
   */
  isWalletConnected(): boolean {
    return walletService.isConnected();
  }

  /**
   * Gets the connected wallet address
   */
  getUserAddress(): string | null {
    return walletService.getWalletAddress();
  }

  /**
   * Gets the vault address for the current user
   */
  getUserVaultAddress(): string | null {
    return tonStateStore.getState().userVaultAddress;
  }

  /**
   * Gets all piece addresses for the current user
   */
  getPieceAddresses(): string[] | null {
    return tonStateStore.getState().pieceAddresses;
  }

  /**
   * Gets data for all pieces
   */
  getPieceData(): { [address: string]: string | null } | null {
    return tonStateStore.getState().pieceData;
  }

  /**
   * Gets remix data for all pieces
   */
  getPieceRemixData(): { [address: string]: string | null } | null {
    return tonStateStore.getState().pieceRemixData || null;
  }

  /**
   * Saves an audio graph to the blockchain
   * @param audioGraphData Audio graph data to save
   */
  async saveAudioGraph(audioGraphData: string): Promise<boolean> {
    console.log('Save audio graph requested:', audioGraphData);
    // TODO: Implement this functionality
    return false;
  }

  /**
   * Loads an audio graph from the blockchain
   * @param nftAddress NFT address to load from
   */
  async loadAudioGraph(nftAddress: string): Promise<string | null> {
    console.log('Load audio graph requested:', nftAddress);
    // TODO: Implement this functionality
    return null;
  }
}

// Export a singleton instance for use across the application
export const tonService = new TonService();
