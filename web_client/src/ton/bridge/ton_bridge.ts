import { Address } from '@ton/core';
import { tonService } from '../services/ton_service';
import { TonBridge } from '../../types';
import { REGISTRY_ADDRESS, FRIENDLY_REGISTRY } from '../../config/constants';

/**
 * Bridge for communication between Rust WASM and JavaScript
 */
export class TonBridgeService {
  private pendingPieceData: { pieceData: string | null; remixedFrom: string | null } = {
    pieceData: null,
    remixedFrom: null,
  };

  /**
   * Initializes the TON bridge
   */
  initialize(): void {
    const bridge: TonBridge = {
      getContractInfo: () => tonService.getContractInfo(),

      registryAddress: () => FRIENDLY_REGISTRY,

      isWalletConnected: (): boolean => tonService.isWalletConnected(),

      getUserAddress: (): string | null => tonService.getUserAddress(),

      getUserVaultAddress: (): string | null => tonService.getUserVaultAddress(),

      getPieceAddresses: (): string[] | null => tonService.getPieceAddresses(),

      getPieceData: (): { [address: string]: string | null } => tonService.getPieceData() || {},

      getPieceRemixData: (): { [address: string]: string | null } | null =>
        tonService.getPieceRemixData(),

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

        const result = await tonService.createNewPiece(pieceRawData, remixedFromAddress);
        return result;
      },

      setPendingPieceData: (
        pieceRawData: string | null,
        remixedFrom: string | null = null,
      ): void => {
        this.pendingPieceData = {
          pieceData: pieceRawData,
          remixedFrom: remixedFrom,
        };
      },

      getPendingPieceData: (): { pieceData: string | null; remixedFrom: string | null } => {
        return this.pendingPieceData;
      },

      clearPendingPieceData: (): void => {
        this.pendingPieceData = {
          pieceData: null,
          remixedFrom: null,
        };
      },
    };

    (window as any).tonBridge = bridge;
  }
}

// Export a singleton instance for use across the application
export const tonBridgeService = new TonBridgeService();
