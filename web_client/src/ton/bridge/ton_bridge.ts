import { Address } from '@ton/core';
import { tonService } from '../services/ton_service';
import { TonBridge } from '../../types';
import { REGISTRY_ADDRESS, FRIENDLY_REGISTRY } from '../../config/constants';

/**
 * Bridge for communication between Rust WASM and JavaScript
 */
export class TonBridgeService {
  // Store pending piece data that needs to be processed
  private pendingPieceData: { pieceData: string | null, remixedFrom: string | null } = {
    pieceData: null,
    remixedFrom: null
  };

  /**
   * Initializes the TON bridge
   */
  initialize(): void {
    // Create the bridge object
    const bridge: TonBridge = {
      getContractInfo: () => tonService.getContractInfo(),

      registryAddress: () => FRIENDLY_REGISTRY,

      isWalletConnected: (): boolean => tonService.isWalletConnected(),

      getUserAddress: (): string | null => tonService.getUserAddress(),

      getUserVaultAddress: (): string | null => tonService.getUserVaultAddress(),

      getPieceAddresses: (): string[] | null => tonService.getPieceAddresses(),

      getPieceData: (): { [address: string]: string | null } => tonService.getPieceData() || {},

      refreshVaultAddress: async (): Promise<string | null> => {
        return tonService.refreshVaultAddress();
      },

      saveAudioGraph: async (audioGraphData: string): Promise<boolean> => {
        return tonService.saveAudioGraph(audioGraphData);
      },

      loadAudioGraph: async (nftAddress: string): Promise<string | null> => {
        return tonService.loadAudioGraph(nftAddress);
      },

      createNewPiece: async (
        pieceRawData: string,
        remixedFrom: string | null = null,
      ): Promise<boolean> => {
        let remixedFromAddress: Address | null = null;

        if (remixedFrom) {
          try {
            remixedFromAddress = Address.parse(remixedFrom);
          } catch (error) {
            console.error('Invalid remixedFrom address:', error);
          }
        }

        console.log('Bridge: Creating new piece...');
        // This will block until the user completes or cancels the transaction
        const result = await tonService.createNewPiece(pieceRawData, remixedFromAddress);
        console.log('Bridge: Transaction result:', result);
        return result;
      },

      // Set pending piece data to be processed by the frontend
      setPendingPieceData: (pieceRawData: string | null, remixedFrom: string | null = null): void => {
        console.log('Setting pending piece data:', pieceRawData);
        this.pendingPieceData = {
          pieceData: pieceRawData,
          remixedFrom: remixedFrom
        };
      },

      // Get the current pending piece data
      getPendingPieceData: (): { pieceData: string | null, remixedFrom: string | null } => {
        return this.pendingPieceData;
      },

      // Clear the pending piece data
      clearPendingPieceData: (): void => {
        this.pendingPieceData = {
          pieceData: null,
          remixedFrom: null
        };
      },
    };

    // Attach the bridge to the window object
    (window as any).tonBridge = bridge;
  }
}

// Export a singleton instance for use across the application
export const tonBridgeService = new TonBridgeService();
