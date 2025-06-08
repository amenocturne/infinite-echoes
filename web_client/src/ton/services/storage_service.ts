import { BaseService } from './base';

const PIECE_DATA_KEY_PREFIX = 'echoes_pieces_';

interface StoredPieceInfo {
  pieceData: { [address: string]: string | null };
  pieceRemixData: { [address: string]: string | null };
}

/**
 * Service for interacting with browser's localStorage
 */
export class StorageService extends BaseService {
  private getStorageKey(userAddress: string): string {
    return `${PIECE_DATA_KEY_PREFIX}${userAddress}`;
  }

  /**
   * Saves piece data for a specific user to localStorage.
   * @param userAddress The address of the user.
   * @param data The data to store.
   */
  savePieces(userAddress: string, data: StoredPieceInfo): void {
    if (!userAddress) return;
    try {
      const key = this.getStorageKey(userAddress);
      const dataToStore = JSON.stringify(data);
      localStorage.setItem(key, dataToStore);
    } catch (error) {
      this.logError('savePieces', error);
      // Handle potential storage errors, e.g., storage full.
    }
  }

  /**
   * Loads piece data for a specific user from localStorage.
   * @param userAddress The address of the user.
   * @returns The stored data, or null if not found or on error.
   */
  loadPieces(userAddress: string): StoredPieceInfo | null {
    if (!userAddress) return null;
    try {
      const key = this.getStorageKey(userAddress);
      const storedData = localStorage.getItem(key);
      if (storedData) {
        return JSON.parse(storedData);
      }
      return null;
    } catch (error) {
      this.logError('loadPieces', error);
      return null;
    }
  }

  /**
   * Clears piece data for a specific user from localStorage.
   * @param userAddress The address of the user.
   */
  clearPieces(userAddress: string): void {
    if (!userAddress) return;
    try {
      const key = this.getStorageKey(userAddress);
      localStorage.removeItem(key);
    } catch (error) {
      this.logError('clearPieces', error);
    }
  }
}

// Export a singleton instance for use across the application
export const storageService = new StorageService();
