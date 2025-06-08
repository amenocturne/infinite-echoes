import { Address, Cell, Dictionary } from '@ton/core';
import { TON_API_TOKEN, TON_TESTNET_API } from '../../config/constants';
import { apiRateLimiter } from './rate_limiter';
import { BaseService } from './base';
import { ErrorCode, TonError, errorHandler } from './error_handler';

/**
 * Service for making API calls to the TON blockchain
 */
export class ApiService extends BaseService {
  /**
   * Calls a getter method on a TON contract with rate limiting
   * @param address Contract address
   * @param method Method name
   * @param args Method arguments
   * @returns Method result
   */
  async callContractGetter(
    address: string,
    method: string,
    args: string[] = [],
  ): Promise<unknown | null> {
    return apiRateLimiter.schedule(async () => {
      try {
        const argsParam =
          args.length > 0 ? `?args=${args.map((arg) => encodeURIComponent(arg)).join(',')}` : '';

        const url = `${TON_TESTNET_API}/${address}/methods/${method}${argsParam}`;

        const response = await fetch(url, {
          method: 'GET',
          headers: {
            Accept: 'application/json',
            Authorization: `Bearer ${TON_API_TOKEN}`,
            'Content-type': 'application/json',
          },
        });

        if (!response.ok) {
          const errorText = await response.text();
          throw new TonError(
            `HTTP error! Status: ${response.status}, Response: ${errorText}`,
            ErrorCode.API_ERROR,
          );
        }

        const data = await response.json();

        if (data.error) {
          throw new TonError(`TON API error: ${data.error}`, ErrorCode.API_ERROR);
        }

        return data;
      } catch (error) {
        throw errorHandler.handleError(error, `callContractGetter(${address}, ${method})`);
      }
    });
  }

  /**
   * Parses a cell from a contract method result
   * @param cellData Cell data in hex format
   * @returns Parsed cell
   */
  parseCell(cellData: string): Cell {
    try {
      // The original implementation was a bit convoluted (hex -> buffer -> base64 -> cell).
      // This is a more direct way: create a buffer from the hex string
      // and then deserialize the BOC (Bag of Cells) from that buffer.
      // `Cell.fromBoc` returns an array of root cells; for a single cell BOC, we take the first one.
      const cellBoc = Buffer.from(cellData, 'hex');
      const cells = Cell.fromBoc(cellBoc);
      return cells[0];
    } catch (error) {
      throw errorHandler.handleError(error, 'parseCell');
    }
  }

  /**
   * Formats an address for display
   * @param address Address to format
   * @returns Formatted address
   */
  formatAddress(address: string): string {
    return address.substring(0, 6) + '...' + address.substring(address.length - 4);
  }
}

// Export a singleton instance for use across the application
export const apiService = new ApiService();
