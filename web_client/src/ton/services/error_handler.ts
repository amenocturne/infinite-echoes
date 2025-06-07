/**
 * Custom error class for TON-related errors
 */
export class TonError extends Error {
  constructor(
    message: string,
    public code: string,
    public originalError?: Error,
  ) {
    super(message);
    this.name = 'TonError';
  }
}

/**
 * Error codes for TON-related errors
 */
export enum ErrorCode {
  API_ERROR = 'API_ERROR',
  WALLET_ERROR = 'WALLET_ERROR',
  CONTRACT_ERROR = 'CONTRACT_ERROR',
  NETWORK_ERROR = 'NETWORK_ERROR',
  UNKNOWN_ERROR = 'UNKNOWN_ERROR',
}

/**
 * Service for handling errors in a consistent way
 */
export class ErrorHandler {
  /**
   * Handles an error and returns a standardized TonError
   * @param error Original error
   * @param context Context in which the error occurred
   * @returns Standardized TonError
   */
  handleError(error: unknown, context: string): TonError {
    if (error instanceof TonError) {
      return error;
    }

    const errorMessage = error instanceof Error ? error.message : String(error);

    const tonError = new TonError(
      `Error in ${context}: ${errorMessage}`,
      ErrorCode.UNKNOWN_ERROR,
      error instanceof Error ? error : undefined,
    );

    console.error(tonError);

    return tonError;
  }

  /**
   * Shows an error message to the user
   * @param error Error to show
   */
  showError(error: unknown): void {
    const message = error instanceof Error ? error.message : String(error);

    alert(message);
  }
}

// Export a singleton instance for use across the application
export const errorHandler = new ErrorHandler();
