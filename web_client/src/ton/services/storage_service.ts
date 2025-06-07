import { BaseService } from './base';

const PIECE_DATA_KEY_PREFIX = 'echoes_pieces_';

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
   * @param pieces A map of piece addresses to their data.
   */
  savePieces(userAddress: string, pieces: { [address: string]: string | null }): void {
    if (!userAddress) return;
    try {
      const key = this.getStorageKey(userAddress);
      const dataToStore = JSON.stringify(pieces);
      localStorage.setItem(key, dataToStore);
    } catch (error) {
      this.logError('savePieces', error);
      // Handle potential storage errors, e.g., storage full.
    }
  }

  /**
   * Loads piece data for a specific user from localStorage.
   * @param userAddress The address of the user.
   * @returns The map of piece data, or null if not found or on error.
   */
  loadPieces(userAddress: string): { [address: string]: string | null } | null {
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
