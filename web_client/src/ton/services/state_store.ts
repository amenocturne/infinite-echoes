import { ContractInfo } from '../../types';

/**
 * Store for managing TON-related state
 */
export class TonStateStore {
  private state: ContractInfo = {
    feeParams: null,
    securityParams: null,
    userVaultAddress: null,
    pieceCount: null,
    pieceAddresses: null,
    pieceData: null,
    pieceRemixData: null,
  };

  private listeners: Array<(state: ContractInfo) => void> = [];
  private isLoading = false;

  /**
   * Gets the current state
   */
  getState(): ContractInfo {
    return { ...this.state };
  }

  /**
   * Updates the state with new values
   * @param newState Partial state to merge with current state
   */
  updateState(newState: Partial<ContractInfo>): void {
    this.state = {
      ...this.state,
      ...newState,
    };
    this.notifyListeners();
  }

  /**
   * Sets the loading state
   */
  setLoading(isLoading: boolean): void {
    this.isLoading = isLoading;
    this.notifyListeners();
  }

  /**
   * Gets the current loading state
   */
  isStateLoading(): boolean {
    return this.isLoading;
  }

  /**
   * Subscribes to state changes
   * @param listener Function to call when state changes
   * @returns Function to unsubscribe
   */
  subscribe(listener: (state: ContractInfo) => void): () => void {
    this.listeners.push(listener);

    // Call listener immediately with current state
    listener(this.getState());

    // Return unsubscribe function
    return () => {
      this.listeners = this.listeners.filter((l) => l !== listener);
    };
  }

  /**
   * Notifies all listeners of state changes
   */
  private notifyListeners(): void {
    const currentState = this.getState();
    this.listeners.forEach((listener) => listener(currentState));
  }
}

// Export a singleton instance for use across the application
export const tonStateStore = new TonStateStore();
