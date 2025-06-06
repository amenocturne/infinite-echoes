import { BaseService } from "./base";
import { apiService } from "./api_service";
import { errorHandler } from "./error_handler";

/**
 * Service for interacting with EchoPiece contracts
 */
export class PieceService extends BaseService {
  /**
   * Gets data from a piece contract as raw base64 encoded data
   * @param pieceAddress Piece address
   * @returns Piece data
   */
  async getPieceData(pieceAddress: string): Promise<string | null> {
    if (!pieceAddress) {
      return null;
    }

    try {
      const result = await apiService.callContractGetter(
        pieceAddress,
        "getData"
      ) as any;

      if (
        !(result && result.success && result.stack && result.stack.length > 0)
      ) {
        return null;
      }

      const cellData = result.stack[0].cell;
      if (!cellData) {
        return null;
      }

      try {
        const cell = apiService.parseCell(cellData);
        const slice = cell.beginParse();
        const result = slice.loadStringTail();
        return result;
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
   * Gets data for all pieces in the provided addresses array
   * @param pieceAddresses Array of piece addresses
   * @returns Object mapping addresses to piece data
   */
  async getAllPieceData(
    pieceAddresses: string[] | null
  ): Promise<{ [address: string]: string | null }> {
    const pieceData: { [address: string]: string | null } = {};

    if (!pieceAddresses || pieceAddresses.length === 0) {
      return pieceData;
    }

    for (let i = 0; i < pieceAddresses.length; i++) {
      const address = pieceAddresses[i];
      try {
        const data = await this.getPieceData(address);
        pieceData[address] = data;
      } catch (error) {
        this.logError(`Processing piece ${address}`, error);
        pieceData[address] = null;
      }
      
      // Add a small delay between requests to avoid rate limiting
      if (i < pieceAddresses.length - 1) {
        await new Promise((resolve) => setTimeout(resolve, 500));
      }
    }

    return pieceData;
  }
}

// Export a singleton instance for use across the application
export const pieceService = new PieceService();
