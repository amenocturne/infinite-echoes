import { Address } from "@ton/core";
import { ContractInfo } from "../../types";
import { BaseService, Initializable } from "./base";
import { errorHandler } from "./error_handler";
import { pieceService } from "./piece_service";
import { registryService } from "./registry_service";
import { tonStateStore } from "./state_store";
import { vaultService } from "./vault_service";
import { walletService } from "./wallet_service";

/**
 * Main service for coordinating TON-related functionality
 */
export class TonService extends BaseService implements Initializable {
  private isLoadingContractInfo = false;

  /**
   * Initializes the TON service
   */
  async initialize(): Promise<void> {
    try {
      // Initialize wallet service
      await walletService.initialize();

      // Subscribe to wallet status changes
      walletService.subscribeToWalletStatus(async (connected) => {
        if (connected) {
          // Refresh contract info when wallet connects
          await this.fetchContractInfo();
        } else {
          // Clear user-specific data when wallet disconnects
          tonStateStore.updateState({
            userVaultAddress: null,
            pieceCount: null,
            pieceAddresses: null,
            pieceData: null,
          });
        }
      });

      // Fetch initial contract info
      await this.fetchContractInfo();
    } catch (error) {
      this.logError("initialize", error);
      throw errorHandler.handleError(error, "TonService.initialize");
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

      // Fetch basic contract info sequentially to respect rate limits
      const feeParams = await registryService.getFeeParams();
      const securityParams = await registryService.getSecurityParams();

      // Update state with basic contract info
      tonStateStore.updateState({
        feeParams,
        securityParams,
      });

      // Check if wallet is connected and fetch vault address
      if (walletService.isConnected()) {
        const userAddress = walletService.getWalletAddress();
        if (userAddress) {
          const vaultAddress = await registryService.getVaultAddress(userAddress);
          tonStateStore.updateState({ userVaultAddress: vaultAddress });

          // Only try to get piece count and addresses if we have a valid vault address
          if (vaultAddress) {
            const pieceCount = await vaultService.getPieceCount(vaultAddress);
            tonStateStore.updateState({ pieceCount });

            const pieceAddresses = await vaultService.getPieceAddresses(vaultAddress);
            tonStateStore.updateState({ pieceAddresses });

            if (pieceAddresses && pieceAddresses.length > 0) {
              const pieceData = await pieceService.getAllPieceData(pieceAddresses);
              tonStateStore.updateState({ pieceData });
            }
          }
        }
      }

      return tonStateStore.getState();
    } catch (error) {
      this.logError("fetchContractInfo", error);
      // Retry after delay
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
      this.logError("refreshVaultAddress", error);
      return null;
    }
  }

  /**
   * Creates a new piece on the blockchain
   * @param pieceRawData Raw data string for the piece
   * @param remixedFrom Optional address this piece was remixed from
   */
  async createNewPiece(
    pieceRawData: string,
    remixedFrom: Address | null = null,
  ): Promise<boolean> {
    try {
      const result = await walletService.createNewPiece(pieceRawData, remixedFrom);
      if (result) {
        // Refresh contract info after successful creation
        setTimeout(() => this.fetchContractInfo(), 5000);
      }
      return result;
    } catch (error) {
      this.logError("createNewPiece", error);
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
