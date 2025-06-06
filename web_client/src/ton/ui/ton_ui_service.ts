import { tonService } from '../services/ton_service';
import { tonStateStore } from '../services/state_store';
import { walletService } from '../services/wallet_service';
import { errorHandler } from '../services/error_handler';

/**
 * Service for handling TON-related UI updates
 */
export class TonUIService {
  private walletStatusElement: HTMLElement | null = null;
  private contractInfoElement: HTMLElement | null = null;
  private connectButton: HTMLButtonElement | null = null;

  /**
   * Initializes the UI service
   */
  initialize(): void {
    // Find UI elements
    this.walletStatusElement = document.getElementById('wallet-status');
    this.contractInfoElement = document.getElementById('contract-info');

    // Create and set up connect button
    this.setupConnectButton();

    // Subscribe to state changes
    tonStateStore.subscribe(() => {
      this.updateContractInfoDisplay();
    });

    // Subscribe to wallet status changes
    walletService.subscribeToWalletStatus((connected) => {
      this.updateWalletStatus(connected);
    });

    // Initial UI update
    this.updateWalletStatus(walletService.isConnected());
    this.updateContractInfoDisplay();
  }

  /**
   * Sets up the wallet connect button
   */
  private setupConnectButton(): void {
    const container = document.getElementById('ton-connect-ui');
    if (!container) {
      console.error('Container #ton-connect-ui not found!');
      return;
    }

    this.connectButton = this.createConnectButton();
    container.innerHTML = '';
    container.appendChild(this.connectButton);

    this.connectButton.addEventListener('click', async () => {
      try {
        if (walletService.isConnected()) {
          await walletService.disconnect();
          this.connectButton!.textContent = 'Connect TON Wallet';
        } else {
          await walletService.openModal();
        }
      } catch (error) {
        errorHandler.handleError(error, 'handleWalletConnection');
        errorHandler.showError('Error handling wallet connection. Please try again.');
      }
    });
  }

  /**
   * Creates a custom wallet connect button
   */
  private createConnectButton(): HTMLButtonElement {
    const customButton = document.createElement('button');
    customButton.id = 'connect-wallet-btn';
    customButton.textContent = 'Connect TON Wallet';
    customButton.style.backgroundColor = '#0088cc';
    customButton.style.color = 'white';
    customButton.style.border = 'none';
    customButton.style.padding = '8px 16px';
    customButton.style.borderRadius = '4px';
    customButton.style.cursor = 'pointer';
    customButton.style.fontWeight = 'bold';
    customButton.style.width = '100%';

    return customButton;
  }

  /**
   * Updates the wallet connection status in the UI
   */
  private updateWalletStatus(connected: boolean): void {
    if (!this.walletStatusElement || !this.connectButton) return;

    if (connected) {
      const address = walletService.getWalletAddress();
      if (address) {
        const formattedAddress = walletService.formatAddress(address);
        this.walletStatusElement.textContent = `Connected: ${formattedAddress}`;
        this.walletStatusElement.classList.add('connected');
        this.connectButton.textContent = 'Disconnect Wallet';
      }
    } else {
      this.walletStatusElement.textContent = 'Not connected';
      this.walletStatusElement.classList.remove('connected');
      this.connectButton.textContent = 'Connect TON Wallet';
    }
  }

  /**
   * Updates the contract info display in the UI
   */
  private updateContractInfoDisplay(): void {
    if (!this.contractInfoElement) {
      return;
    }

    if (tonStateStore.isStateLoading()) {
      this.contractInfoElement.innerHTML =
        '<div>Loading contract info... <span class="loading-spinner"></span></div>';
      return;
    }

    const contractInfo = tonStateStore.getState();
    let html = ``;

    // Add vault address if available
    if (walletService.isConnected()) {
      if (contractInfo.userVaultAddress) {
        const shortAddress = walletService.formatAddress(contractInfo.userVaultAddress);
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
            html += `<div>- ${walletService.formatAddress(address)}`;

            // Add piece data if available
            if (contractInfo.pieceData && contractInfo.pieceData[address]) {
              html += ` (Base64 Data: ${
                contractInfo.pieceData[address]!.substring(0, 10)
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

    this.contractInfoElement.innerHTML = html;
  }
}

// Export a singleton instance for use across the application
export const tonUIService = new TonUIService();
