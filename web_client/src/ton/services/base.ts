/**
 * Base class for all TON-related services
 */
export abstract class BaseService {
  protected logError(method: string, error: unknown): void {
    console.error(`[${this.constructor.name}] Error in ${method}:`, error);
  }
}

/**
 * Interface for services that require initialization
 */
export interface Initializable {
  initialize(): Promise<void>;
}
