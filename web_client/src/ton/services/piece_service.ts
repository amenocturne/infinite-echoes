import { BaseService } from './base';
import { apiService } from './api_service';
import { tonStateStore } from './state_store';
import { storageService } from './storage_service';
import { Cell } from '@ton/core';

/**
 * Service for interacting with EchoPiece contracts
 */
export class PieceService extends BaseService {
  /**
   * Gets data from a piece contract as raw base64 encoded data,
   * traversing the linked list of cells if necessary.
   * @param pieceAddress Piece address
   * @returns Piece data as a single base64 string
   */
  async getPieceData(pieceAddress: string): Promise<string | null> {
    if (!pieceAddress) {
      return null;
    }

    try {
      const result = (await apiService.callContractGetter(pieceAddress, 'getData')) as any;

      if (!(result && result.success && result.stack && result.stack.length > 0)) {
        return null;
      }

      const headCellData = result.stack[0].cell;
      if (!headCellData) {
        return null;
      }

      try {
        let currentCell: Cell | null = apiService.parseCell(headCellData);
        let fullDataBuffer = Buffer.alloc(0);

        while (currentCell) {
          const slice = currentCell.beginParse();

          // The data is stored using `storeBuffer`, which writes the raw bytes without a length prefix.
          // We need to read all the available bytes from the slice.
          const chunk = slice.loadBuffer(Math.floor(slice.remainingBits / 8));
          fullDataBuffer = Buffer.concat([fullDataBuffer, chunk]);

          // Move to the next cell in the linked list, if it exists.
          if (slice.remainingRefs > 0) {
            currentCell = slice.loadRef();
          } else {
            currentCell = null;
          }
        }

        return fullDataBuffer.toString('base64');
      } catch (parseError) {
        this.logError(`Processing piece data for ${pieceAddress}`, parseError);
        return null;
      }
    } catch (error) {
      this.logError(`Getting piece data for ${pieceAddress}`, error);
      return null;
    }
  }

  /**
   * Gets the "remixed from" address from a piece contract.
   * @param pieceAddress Piece address
   * @returns The address the piece was remixed from, or null.
   */
  async getRemixedFromAddress(pieceAddress: string): Promise<string | null> {
    if (!pieceAddress) {
      return null;
    }

    try {
      const result = (await apiService.callContractGetter(pieceAddress, 'getRemixedFrom')) as any;

      if (result && result.success && result.stack && result.stack.length > 0) {
        const stackItem = result.stack[0];
        // For Address?, a null value is represented by a 'null' type in the stack.
        if (stackItem.type === 'null') {
          return null;
        }

        // If it's not null, it should be a cell.
        if (stackItem.type === 'cell' && stackItem.cell) {
          const cell = apiService.parseCell(stackItem.cell);
          const slice = cell.beginParse();
          return slice.loadAddress().toString({ testOnly: false, bounceable: true });
        }
      }
      return null;
    } catch (error) {
      this.logError(`Getting remixedFrom for ${pieceAddress}`, error);
      return null;
    }
  }

  /**
   * Fetches data for all pieces, updates the state store, and caches the result.
   * @param userAddress The address of the current user for caching.
   * @param pieceAddresses Array of piece addresses to fetch.
   */
  fetchAllPieceData(userAddress: string, pieceAddresses: string[] | null): void {
    if (!userAddress || !pieceAddresses || pieceAddresses.length === 0) {
      return;
    }

    pieceAddresses.forEach(async (address) => {
      try {
        // Fetch both data and remix info in parallel
        const [data, remixedFrom] = await Promise.all([
          this.getPieceData(address),
          this.getRemixedFromAddress(address),
        ]);

        const currentState = tonStateStore.getState();
        const newPieceData = { ...(currentState.pieceData || {}), [address]: data };
        const newRemixData = { ...(currentState.pieceRemixData || {}), [address]: remixedFrom };

        tonStateStore.updateState({ pieceData: newPieceData, pieceRemixData: newRemixData });
        storageService.savePieces(userAddress, {
          pieceData: newPieceData,
          pieceRemixData: newRemixData,
        }); // Cache the updated data
      } catch (error) {
        this.logError(`Processing piece ${address}`, error);
        const currentState = tonStateStore.getState();
        // Store null to indicate a fetch attempt failed for this address
        const newPieceData = { ...(currentState.pieceData || {}), [address]: null };
        const newRemixData = { ...(currentState.pieceRemixData || {}), [address]: null };
        tonStateStore.updateState({ pieceData: newPieceData, pieceRemixData: newRemixData });
        storageService.savePieces(userAddress, {
          pieceData: newPieceData,
          pieceRemixData: newRemixData,
        }); // Also cache failures
      }
    });
  }
}

// Export a singleton instance for use across the application
export const pieceService = new PieceService();
