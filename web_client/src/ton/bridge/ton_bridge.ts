import { Address } from '@ton/core';
import { tonService } from '../services/ton_service';
import { TonBridge } from '../../types';

/**
 * Bridge for communication between Rust WASM and JavaScript
 */
export class TonBridgeService {
  /**
   * Initializes the TON bridge
   */
  initialize(): void {
    // Create the bridge object
    const bridge: TonBridge = {
      getContractInfo: () => tonService.getContractInfo(),
      
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
        remixedFrom: string | null = null
      ): Promise<boolean> => {
        let remixedFromAddress: Address | null = null;
        
        if (remixedFrom) {
          try {
            remixedFromAddress = Address.parse(remixedFrom);
          } catch (error) {
            console.error('Invalid remixedFrom address:', error);
          }
        }
        
        return tonService.createNewPiece(pieceRawData, remixedFromAddress);
      },
    };

    // Attach the bridge to the window object
    (window as any).tonBridge = bridge;
  }
}

// Export a singleton instance for use across the application
export const tonBridgeService = new TonBridgeService();
