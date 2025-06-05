import { TON_API_TOKEN, TON_TESTNET_API, REGISTRY_ADDRESS } from '../config/constants';
import { ContractInfo } from '../types';

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
      await new Promise(resolve => setTimeout(resolve, timeToWait));
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
      console.log(`Calling TON API: ${method}`);

      // Format arguments for URL
      const argsParam = args.length > 0
        ? `?args=${args.map(arg => encodeURIComponent(arg)).join(',')}`
        : '';

      const url = `${TON_TESTNET_API}/${address}/methods/${method}${argsParam}`;
      console.log('API URL:', url);

      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
          'Authorization': `Bearer ${TON_API_TOKEN}`,
          'Content-type': 'application/json'
        }
      });

      if (!response.ok) {
        const errorText = await response.text();
        console.error(`HTTP error! Status: ${response.status}, Response: ${errorText}`);
        throw new Error(`HTTP error! Status: ${response.status}, Response: ${errorText}`);
      }

      const data = await response.json();
      console.log(`API response for ${method}:`, data);

      if (data.error) {
        console.error('TON API error:', data.error);
        return null;
      }

      return data;
    } catch (error) {
      console.error('Error calling TON contract:', error);
      return null;
    }
  });
}

/**
 * Gets the current piece version from the registry contract
 */
export async function getPieceVersion(): Promise<number | null> {
  const result = (await callContractGetter(REGISTRY_ADDRESS, 'getPieceVersion')) as any;
  console.log('getPieceVersion raw result:', result);
  if (result && result.success && result.stack[0] && result.stack[0].num) {
    return parseInt(result.stack[0].num, 16);
  }
  return null;
}

/**
 * Gets the current vault version from the registry contract
 */
export async function getVaultVersion(): Promise<number | null> {
  const result = (await callContractGetter(REGISTRY_ADDRESS, 'getVaultVersion')) as any;
  if (result && result.success && result.stack[0] && result.stack[0].num) {
    return parseInt(result.stack[0].num, 16);
  }
  return null;
}

/**
 * Gets the fee parameters from the registry contract
 */
export async function getFeeParams(): Promise<{
  deployValue: number;
  messageValue: number;
} | null> {
  const result = (await callContractGetter(REGISTRY_ADDRESS, 'getFeeParams')) as any;
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
export async function getSecurityParams(): Promise<{
  minActionFee: number;
  coolDownSeconds: number;
} | null> {
  const result = (await callContractGetter(REGISTRY_ADDRESS, 'getSecurityParams')) as any;
  if (result && result.success && result.stack[0] && result.stack[1]) {
    const minActionFee = parseInt(result.stack[0].num, 16);
    const coolDownSeconds = parseInt(result.stack[1].num, 16);
    return {
      minActionFee,
      coolDownSeconds
    };
  }
  return null;
}

/**
 * Gets the vault address for a specific user
 */
export async function getVaultAddress(userAddress: string): Promise<string | null> {
  // Make sure the address is in the correct format (0:...)
  const formattedAddress = userAddress.startsWith('0:') ? userAddress : `0:${userAddress}`;
  console.log('Getting vault address for user:', formattedAddress);

  const result = (await callContractGetter(REGISTRY_ADDRESS, 'getVaultAddress', [formattedAddress])) as any;
  console.log('getVaultAddress raw result:', result);

  if (result && result.stack[0] && result.stack[0].type == "cell") {
    const addr = result.stack[0].cell;
    console.log('Found vault address:', addr);
    return addr;
  }
  return null;
}

// Track loading state
let isLoadingContractInfo = false;

export let contractInfo: ContractInfo = {
  pieceVersion: null,
  vaultVersion: null,
  feeParams: null,
  securityParams: null,
  userVaultAddress: null,
};

/**
 * Fetches all contract information from the registry
 */
export async function fetchContractInfo(): Promise<ContractInfo | null> {
  if (isLoadingContractInfo) {
    console.log('Contract info already loading, skipping duplicate request');
    return null;
  }

  try {
    isLoadingContractInfo = true;
    updateContractInfoDisplay();

    // Fetch basic contract info sequentially to respect rate limits
    const pieceVersion = await getPieceVersion();
    const vaultVersion = await getVaultVersion();
    const feeParams = await getFeeParams();
    const securityParams = await getSecurityParams();

    // Initialize with basic contract info
    contractInfo = {
      pieceVersion,
      vaultVersion,
      feeParams,
      securityParams,
      userVaultAddress: null,
    };

    // Update display with initial info
    updateContractInfoDisplay();

    // Check if wallet is connected and fetch vault address
    if (window.tonBridge && window.tonBridge.isWalletConnected()) {
      const userAddress = window.tonBridge.getUserAddress();
      if (userAddress) {
        const vaultAddress = await getVaultAddress(userAddress);
        contractInfo.userVaultAddress = vaultAddress;
      }
    }

    console.log('Contract info loaded:', contractInfo);
    return contractInfo;
  } catch (error) {
    console.error('Error fetching contract info:', error);
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
  const contractInfoElement = document.getElementById('contract-info');
  if (!contractInfoElement) return;

  if (isLoadingContractInfo) {
    contractInfoElement.innerHTML = '<div>Loading contract info... <span class="loading-spinner"></span></div>';
    return;
  }

  if (contractInfo.pieceVersion !== null) {
    let html = `
      <div>Piece Version: ${contractInfo.pieceVersion}</div>
      ${
        contractInfo.vaultVersion !== null
          ? `<div>Vault Version: ${contractInfo.vaultVersion}</div>`
          : ''
      }
    `;

    // Add vault address if available
    if (window.tonBridge && window.tonBridge.isWalletConnected()) {
      if (contractInfo.userVaultAddress) {
        const shortAddress = formatAddress(contractInfo.userVaultAddress);
        html += `<div>Your Vault: ${shortAddress}</div>`;
      } else {
        html += `<div>Your Vault: Not created yet</div>`;
      }
    }

    contractInfoElement.innerHTML = html;
  } else {
    contractInfoElement.innerHTML = '<div>Loading contract info... <span class="loading-spinner"></span></div>';
  }
}

/**
 * Formats an address for display
 */
function formatAddress(address: string): string {
  return address.substring(0, 6) + '...' + address.substring(address.length - 4);
}
