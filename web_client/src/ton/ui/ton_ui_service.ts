import { tonService } from '../services/ton_service';
import { tonStateStore } from '../services/state_store';
import { walletService } from '../services/wallet_service';
import { errorHandler } from '../services/error_handler';

/**
 * Service for handling TON-related UI updates
 */
export class TonUIService {
  private connectButton: HTMLButtonElement | null = null;

  /**
   * Initializes the UI service
   */
  initialize(): void {
    this.setupConnectButton();

    tonStateStore.subscribe(() => {
      // No longer updating contract info display here
    });

    walletService.subscribeToWalletStatus((connected) => {
      this.updateConnectButtonText(connected);
    });

    this.updateConnectButtonText(walletService.isConnected());
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
   * Updates the connect button text based on wallet connection status
   */
  private updateConnectButtonText(connected: boolean): void {
    if (!this.connectButton) return;

    if (connected) {
      this.connectButton.textContent = 'Disconnect Wallet';
    } else {
      this.connectButton.textContent = 'Connect TON Wallet';
    }
  }
}

// Export a singleton instance for use across the application
export const tonUIService = new TonUIService();
