import {
  REGISTRY_ADDRESS,
  TON_API_TOKEN,
  TON_TESTNET_API,
} from "../config/constants";
import { ContractInfo } from "../types";
import { Address, Cell, Dictionary } from "@ton/core";

// Rate limiter for TON API calls
class ApiRateLimiter {
  private queue: Array<() => Promise<void>> = [];
  private processing = false;
  private lastCallTime = 0;
  private minInterval = 1000; // 1 second between calls

  async schedule<T>(fn: () => Promise<T>): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      this.queue.push(async () => {
        try {
          const result = await fn();
          resolve(result);
        } catch (error) {
          reject(error);
        }
      });

      if (!this.processing) {
        this.processQueue();
      }
    });
  }

  private async processQueue() {
    if (this.queue.length === 0) {
      this.processing = false;
      return;
    }

    this.processing = true;

    // Ensure minimum time between requests
    const now = Date.now();
    const timeToWait = Math.max(0, this.lastCallTime + this.minInterval - now);

    if (timeToWait > 0) {
      await new Promise((resolve) => setTimeout(resolve, timeToWait));
    }

    const task = this.queue.shift();
    if (task) {
      this.lastCallTime = Date.now();
      try {
        await task();
      } catch (error) {
        console.error("Error in rate-limited task:", error);
      }
    }

    // Process next item in queue
    this.processQueue();
  }
}

const apiRateLimiter = new ApiRateLimiter();

/**
 * Calls a getter method on a TON contract with rate limiting
 */
async function callContractGetter(
  address: string,
  method: string,
  args: string[] = [],
): Promise<unknown | null> {
  return apiRateLimiter.schedule(async () => {
    try {
      // Format arguments for URL
      const argsParam = args.length > 0
        ? `?args=${args.map((arg) => encodeURIComponent(arg)).join(",")}`
        : "";

      const url = `${TON_TESTNET_API}/${address}/methods/${method}${argsParam}`;

      const response = await fetch(url, {
        method: "GET",
        headers: {
          Accept: "application/json",
          Authorization: `Bearer ${TON_API_TOKEN}`,
          "Content-type": "application/json",
        },
      });

      if (!response.ok) {
        const errorText = await response.text();
        console.error(
          `HTTP error! Status: ${response.status}, Response: ${errorText}`,
        );
        throw new Error(
          `HTTP error! Status: ${response.status}, Response: ${errorText}`,
        );
      }

      const data = await response.json();

      if (data.error) {
        console.error("TON API error:", data.error);
        return null;
      }

      return data;
    } catch (error) {
      console.error("Error calling TON contract:", error);
      return null;
    }
  });
}

/**
 * Gets the fee parameters from the registry contract
 */
export async function getFeeParams(): Promise<
  {
    deployValue: number;
    messageValue: number;
  } | null
> {
  const result =
    (await callContractGetter(REGISTRY_ADDRESS, "getFeeParams")) as any;
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
}

/**
 * Gets the security parameters from the registry contract
 */
export async function getSecurityParams(): Promise<
  {
    minActionFee: number;
    coolDownSeconds: number;
  } | null
> {
  const result =
    (await callContractGetter(REGISTRY_ADDRESS, "getSecurityParams")) as any;
  if (result && result.success && result.stack[0] && result.stack[1]) {
    const minActionFee = parseInt(result.stack[0].num, 16);
    const coolDownSeconds = parseInt(result.stack[1].num, 16);
    return {
      minActionFee,
      coolDownSeconds,
    };
  }
  return null;
}

/**
 * Gets the vault address for a specific user
 */
export async function getVaultAddress(
  userAddress: string,
): Promise<string | null> {
  // Make sure the address is in the correct format (0:...)
  const formattedAddress = userAddress.startsWith("0:")
    ? userAddress
    : `0:${userAddress}`;

  const result =
    (await callContractGetter(REGISTRY_ADDRESS, "getVaultAddress", [
      formattedAddress,
    ])) as any;

  if (result && result.stack[0] && result.stack[0].type == "cell") {
    const cellData = result.stack[0].cell;
    const cell = Cell.fromBase64(
      Buffer.from(cellData, "hex").toString("base64"),
    );
    const slice = cell.beginParse();
    const addr = slice.loadAddress().toString({
      testOnly: false, // Set to true for testnet
      bounceable: true, // Set to false for non-bounceable
    });
    return addr;
  }
  return null;
}

/**
 * Gets the piece count from a user's vault
 */
export async function getPieceCount(
  vaultAddress: string | null,
): Promise<number | null> {
  // Check if vaultAddress is valid
  if (!vaultAddress) {
    return null;
  }

  // First try with the raw address format
  try {
    const result =
      (await callContractGetter(vaultAddress, "getPieceCount")) as any;

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
  } catch (directError) {
    console.error("Error with direct address format:", directError);
  }

  // If direct approach fails, try with friendly address format
  try {
    // Parse the raw address and convert to friendly format
    const parts = vaultAddress.split(":");
    if (parts.length !== 2) {
      console.error(
        "Invalid address format for friendly conversion:",
        vaultAddress,
      );
      return null;
    }

    const workchain = parseInt(parts[0]);
    const addressPart = parts[1];

    if (!addressPart) {
      console.error(
        "Invalid address part for friendly conversion:",
        vaultAddress,
      );
      return null;
    }

    // Create Address object and convert to friendly format
    const address = new Address(workchain, Buffer.from(addressPart, "hex"));
    const friendlyAddress = address.toString({
      testOnly: true, // Using testOnly: true for testnet
      bounceable: true,
    });

    const result =
      (await callContractGetter(friendlyAddress, "getPieceCount")) as any;

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
    console.error("Error with friendly address format:", friendlyError);
  }

  return null;
}

/**
 * Gets all piece addresses from a user's vault
 */
export async function getPieceAddresses(
  vaultAddress: string | null,
): Promise<string[] | null> {
  // Check if vaultAddress is valid
  if (!vaultAddress) {
    return null;
  }

  // First try with the raw address format
  try {
    const result = (await callContractGetter(vaultAddress, "getPieces")) as any;

    if (result && result.success && result.stack && result.stack.length > 0) {
      // The result is a dictionary of piece addresses
      const dictCell = result.stack[0].cell;
      if (!dictCell) {
        return [];
      }

      try {
        // Parse the dictionary using Dictionary utility
        const cell = Cell.fromBase64(
          Buffer.from(dictCell, "hex").toString("base64"),
        );

        // Load the dictionary with uint16 keys and Address values
        const dict = Dictionary.loadDirect(
          Dictionary.Keys.Uint(16),
          Dictionary.Values.Address(),
          cell,
        );

        // Convert dictionary to array of addresses
        const addresses: string[] = [];
        for (const [_, value] of dict) {
          addresses.push(value.toString({
            testOnly: false,
            bounceable: true,
          }));
        }

        return addresses;
      } catch (parseError) {
        console.error("Error parsing piece addresses:", parseError);
        return [];
      }
    }
  } catch (directError) {
    console.error("Error with direct address format:", directError);
  }

  // If direct approach fails, try with friendly address format
  try {
    // Parse the raw address and convert to friendly format
    const parts = vaultAddress.split(":");
    if (parts.length !== 2) {
      console.error(
        "Invalid address format for friendly conversion:",
        vaultAddress,
      );
      return null;
    }

    const workchain = parseInt(parts[0]);
    const addressPart = parts[1];

    if (!addressPart) {
      console.error(
        "Invalid address part for friendly conversion:",
        vaultAddress,
      );
      return null;
    }

    // Create Address object and convert to friendly format
    const address = new Address(workchain, Buffer.from(addressPart, "hex"));
    const friendlyAddress = address.toString({
      testOnly: true, // Using testOnly: true for testnet
      bounceable: true,
    });

    const result =
      (await callContractGetter(friendlyAddress, "getPieces")) as any;

    if (result && result.success && result.stack && result.stack.length > 0) {
      const dictCell = result.stack[0].cell;
      if (!dictCell) {
        return [];
      }

      try {
        // Parse the dictionary using Dictionary utility
        const cell = Cell.fromBase64(
          Buffer.from(dictCell, "hex").toString("base64"),
        );

        // Load the dictionary with uint16 keys and Address values
        const dict = Dictionary.loadDirect(
          Dictionary.Keys.Uint(16),
          Dictionary.Values.Address(),
          cell,
        );

        // Convert dictionary to array of addresses
        const addresses: string[] = [];
        for (const [_, value] of dict) {
          addresses.push(value.toString({
            testOnly: true, // Using testOnly: true for testnet
            bounceable: true,
          }));
        }

        return addresses;
      } catch (parseError) {
        console.error("Error parsing piece addresses:", parseError);
        return [];
      }
    }
  } catch (friendlyError) {
    console.error("Error with friendly address format:", friendlyError);
  }

  return null;
}

/**
 * Gets data from a piece contract as raw base64 encoded data
 */
export async function getPieceData(
  pieceAddress: string,
): Promise<string | null> {
  if (!pieceAddress) {
    return null;
  }

  try {
    const result = (await callContractGetter(pieceAddress, "getData")) as any;

    if (
      !(result && result.success && result.stack && result.stack.length > 0)
    ) {
      console.log("Api did not return correct data");
      return null;
    }

    const cellData = result.stack[0].cell;
    if (!cellData) {
      console.log("No data");
      return null;
    }

    try {
      const base64Data = Buffer.from(cellData, "hex").toString("base64");
      const slice = Cell.fromBase64(base64Data).beginParse();
      const result = slice.loadStringTail();
      console.log("Got result");
      return result;
    } catch (parseError) {
      console.error(
        `Error processing piece data for ${pieceAddress}:`,
        parseError,
      );
      return null;
    }
  } catch (error) {
    console.error(`Error getting piece data for ${pieceAddress}:`, error);
  }

  console.log("Here");
  return null;
}

/**
 * Gets data for all pieces in the provided addresses array
 */
export async function getAllPieceData(
  pieceAddresses: string[] | null,
): Promise<{ [address: string]: string | null }> {
  const pieceData: { [address: string]: string | null } = {};

  if (!pieceAddresses || pieceAddresses.length === 0) {
    return pieceData;
  }

  for (let i = 0; i < pieceAddresses.length; i++) {
    const address = pieceAddresses[i];
    try {
      const data = await getPieceData(address);
      pieceData[address] = data;
    } catch (error) {
      console.error(`Error processing piece ${address}:`, error);
    }
    if (i < pieceAddresses.length - 1) {
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  }

  return pieceData;
}

// Track loading state
let isLoadingContractInfo = false;

export let contractInfo: ContractInfo = {
  feeParams: null,
  securityParams: null,
  userVaultAddress: null,
  pieceCount: null,
  pieceAddresses: null,
  pieceData: null,
};

/**
 * Fetches all contract information from the registry
 */
export async function fetchContractInfo(): Promise<ContractInfo | null> {
  if (isLoadingContractInfo) {
    return null;
  }

  try {
    isLoadingContractInfo = true;
    updateContractInfoDisplay();

    // Fetch basic contract info sequentially to respect rate limits
    const feeParams = await getFeeParams();
    const securityParams = await getSecurityParams();

    // Initialize with basic contract info
    contractInfo = {
      feeParams,
      securityParams,
      userVaultAddress: null,
      pieceCount: null,
      pieceAddresses: null,
      pieceData: null,
    };

    // Update display with initial info
    updateContractInfoDisplay();

    // Check if wallet is connected and fetch vault address
    if (window.tonBridge && window.tonBridge.isWalletConnected()) {
      const userAddress = window.tonBridge.getUserAddress();
      if (userAddress) {
        const vaultAddress = await getVaultAddress(userAddress);
        contractInfo.userVaultAddress = vaultAddress;

        // Only try to get piece count and addresses if we have a valid vault address
        if (vaultAddress) {
          const pieceCount = await getPieceCount(vaultAddress);
          contractInfo.pieceCount = pieceCount;

          const pieceAddresses = await getPieceAddresses(vaultAddress);
          contractInfo.pieceAddresses = pieceAddresses;
          const pieceData = await getAllPieceData(pieceAddresses);
          contractInfo.pieceData = pieceData;
        } else {
          contractInfo.pieceCount = null;
          contractInfo.pieceAddresses = null;
        }
      }
    }

    return contractInfo;
  } catch (error) {
    console.error("Error fetching contract info:", error);
    // Retry after delay
    setTimeout(() => fetchContractInfo(), 3000);
    return null;
  } finally {
    isLoadingContractInfo = false;
    updateContractInfoDisplay();
  }
}

/**
 * Updates the contract info display in the UI
 */
export function updateContractInfoDisplay(): void {
  const contractInfoElement = document.getElementById("contract-info");
  if (!contractInfoElement) {
    return;
  }

  if (isLoadingContractInfo) {
    contractInfoElement.innerHTML =
      '<div>Loading contract info... <span class="loading-spinner"></span></div>';
    return;
  }

  let html = ``;

  // Add vault address if available
  if (window.tonBridge && window.tonBridge.isWalletConnected()) {
    if (contractInfo.userVaultAddress) {
      const shortAddress = formatAddress(contractInfo.userVaultAddress);
      html += `<div>Your Vault: ${shortAddress}</div>`;

      // Add piece count if available
      if (contractInfo.pieceCount !== null) {
        html += `<div>Your Pieces: ${contractInfo.pieceCount}</div>`;
      }

      // Add piece addresses if available
      if (
        contractInfo.pieceAddresses && contractInfo.pieceAddresses.length > 0
      ) {
        html +=
          `<div>Piece Addresses: ${contractInfo.pieceAddresses.length} found</div>`;
        // Optionally show the first few addresses
        const maxToShow = Math.min(3, contractInfo.pieceAddresses.length);
        for (let i = 0; i < maxToShow; i++) {
          const address = contractInfo.pieceAddresses[i];
          html += `<div>- ${formatAddress(address)}`;

          // Add piece data if available
          if (contractInfo.pieceData && contractInfo.pieceData[address]) {
            html += ` (Base64 Data: ${
              contractInfo.pieceData[address].substring(0, 10)
            }...)`;
          }

          html += `</div>`;
        }
        if (contractInfo.pieceAddresses.length > maxToShow) {
          html += `<div>...and ${
            contractInfo.pieceAddresses.length - maxToShow
          } more</div>`;
        }
      }
    } else {
      html += `<div>Your Vault: Not created yet</div>`;
    }
  }

  contractInfoElement.innerHTML = html;
}

/**
 * Formats an address for display
 */
function formatAddress(address: string): string {
  return address.substring(0, 6) + "..." +
    address.substring(address.length - 4);
}
