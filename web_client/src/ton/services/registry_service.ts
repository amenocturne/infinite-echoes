import { Address } from "@ton/core";
import { REGISTRY_ADDRESS } from "../../config/constants";
import { BaseService } from "./base";
import { apiService } from "./api_service";
import { errorHandler } from "./error_handler";

/**
 * Service for interacting with the EchoRegistry contract
 */
export class RegistryService extends BaseService {
  /**
   * Gets the fee parameters from the registry contract
   * @returns Fee parameters
   */
  async getFeeParams(): Promise<{
    deployValue: number;
    messageValue: number;
  } | null> {
    try {
      const result = await apiService.callContractGetter(
        REGISTRY_ADDRESS,
        "getFeeParams"
      ) as any;

      if (result && result.success && result.stack[0] && result.stack[1]) {
        // The result is a tuple with deployValue and messageValue
        const deployValue = parseInt(result.stack[0].num, 16);
        const messageValue = parseInt(result.stack[1].num, 16);
        return {
          deployValue,
          messageValue,
        };
      }
      return null;
    } catch (error) {
      this.logError("getFeeParams", error);
      return null;
    }
  }

  /**
   * Gets the security parameters from the registry contract
   * @returns Security parameters
   */
  async getSecurityParams(): Promise<{
    minActionFee: number;
    coolDownSeconds: number;
  } | null> {
    try {
      const result = await apiService.callContractGetter(
        REGISTRY_ADDRESS,
        "getSecurityParams"
      ) as any;

      if (result && result.success && result.stack[0] && result.stack[1]) {
        const minActionFee = parseInt(result.stack[0].num, 16);
        const coolDownSeconds = parseInt(result.stack[1].num, 16);
        return {
          minActionFee,
          coolDownSeconds,
        };
      }
      return null;
    } catch (error) {
      this.logError("getSecurityParams", error);
      return null;
    }
  }

  /**
   * Gets the vault address for a specific user
   * @param userAddress User address
   * @returns Vault address
   */
  async getVaultAddress(userAddress: string): Promise<string | null> {
    try {
      // Make sure the address is in the correct format (0:...)
      const formattedAddress = userAddress.startsWith("0:")
        ? userAddress
        : `0:${userAddress}`;

      const result = await apiService.callContractGetter(
        REGISTRY_ADDRESS,
        "getVaultAddress",
        [formattedAddress]
      ) as any;

      if (result && result.stack[0] && result.stack[0].type == "cell") {
        const cellData = result.stack[0].cell;
        const cell = apiService.parseCell(cellData);
        const slice = cell.beginParse();
        const addr = slice.loadAddress().toString({
          testOnly: false, // Set to true for testnet
          bounceable: true, // Set to false for non-bounceable
        });
        return addr;
      }
      return null;
    } catch (error) {
      this.logError("getVaultAddress", error);
      return null;
    }
  }
}

// Export a singleton instance for use across the application
export const registryService = new RegistryService();
