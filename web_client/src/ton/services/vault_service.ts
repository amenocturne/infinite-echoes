import { Address, Dictionary } from '@ton/core';
import { BaseService } from './base';
import { apiService } from './api_service';
import { errorHandler } from './error_handler';

/**
 * Service for interacting with the EchoVault contract
 */
export class VaultService extends BaseService {
  /**
   * Gets the piece count from a user's vault
   * @param vaultAddress Vault address
   * @returns Piece count
   */
  async getPieceCount(vaultAddress: string | null): Promise<number | null> {
    if (!vaultAddress) {
      return null;
    }

    try {
      const result = (await apiService.callContractGetter(vaultAddress, 'getPieceCount')) as any;

      if (
        result &&
        result.success &&
        result.stack &&
        result.stack.length > 0 &&
        result.stack[0].num
      ) {
        const count = parseInt(result.stack[0].num, 16);
        return count;
      }

      return null;
    } catch (error) {
      this.logError('getPieceCount', error);

      try {
        const parts = vaultAddress.split(':');
        if (parts.length !== 2) {
          return null;
        }

        const workchain = parseInt(parts[0]);
        const addressPart = parts[1];

        if (!addressPart) {
          return null;
        }

        const address = new Address(workchain, Buffer.from(addressPart, 'hex'));
        const friendlyAddress = address.toString({
          testOnly: true,
          bounceable: true,
        });

        const result = (await apiService.callContractGetter(
          friendlyAddress,
          'getPieceCount',
        )) as any;

        if (
          result &&
          result.success &&
          result.stack &&
          result.stack.length > 0 &&
          result.stack[0].num
        ) {
          const count = parseInt(result.stack[0].num, 16);
          return count;
        }
      } catch (friendlyError) {
        this.logError('getPieceCount (friendly format)', friendlyError);
      }

      return null;
    }
  }

  /**
   * Gets all piece addresses from a user's vault
   * @param vaultAddress Vault address
   * @returns Array of piece addresses
   */
  async getPieceAddresses(vaultAddress: string | null): Promise<string[] | null> {
    if (!vaultAddress) {
      return null;
    }

    try {
      const result = (await apiService.callContractGetter(vaultAddress, 'getPieces')) as any;

      if (result && result.success && result.stack && result.stack.length > 0) {
        const dictCell = result.stack[0].cell;
        if (!dictCell) {
          return [];
        }

        try {
          const cell = apiService.parseCell(dictCell);

          const dict = Dictionary.loadDirect(
            Dictionary.Keys.Uint(16),
            Dictionary.Values.Address(),
            cell,
          );

          const addresses: string[] = [];
          for (const [_, value] of dict) {
            addresses.push(
              value.toString({
                testOnly: false,
                bounceable: true,
              }),
            );
          }

          return addresses;
        } catch (parseError) {
          this.logError('Parsing piece addresses', parseError);
          return [];
        }
      }

      return [];
    } catch (error) {
      this.logError('getPieceAddresses', error);

      try {
        const parts = vaultAddress.split(':');
        if (parts.length !== 2) {
          return null;
        }

        const workchain = parseInt(parts[0]);
        const addressPart = parts[1];

        if (!addressPart) {
          return null;
        }

        const address = new Address(workchain, Buffer.from(addressPart, 'hex'));
        const friendlyAddress = address.toString({
          testOnly: true,
          bounceable: true,
        });

        const result = (await apiService.callContractGetter(friendlyAddress, 'getPieces')) as any;

        if (result && result.success && result.stack && result.stack.length > 0) {
          const dictCell = result.stack[0].cell;
          if (!dictCell) {
            return [];
          }

          try {
            const cell = apiService.parseCell(dictCell);

            const dict = Dictionary.loadDirect(
              Dictionary.Keys.Uint(16),
              Dictionary.Values.Address(),
              cell,
            );

            const addresses: string[] = [];
            for (const [_, value] of dict) {
              addresses.push(
                value.toString({
                  testOnly: true,
                  bounceable: true,
                }),
              );
            }

            return addresses;
          } catch (parseError) {
            this.logError('Parsing piece addresses (friendly format)', parseError);
            return [];
          }
        }
      } catch (friendlyError) {
        this.logError('getPieceAddresses (friendly format)', friendlyError);
      }

      return null;
    }
  }
}

// Export a singleton instance for use across the application
export const vaultService = new VaultService();
