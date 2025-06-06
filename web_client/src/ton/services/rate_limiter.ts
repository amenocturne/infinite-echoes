/**
 * Service for rate limiting API calls to TON blockchain
 */
export class ApiRateLimiter {
  private queue: Array<() => Promise<void>> = [];
  private processing = false;
  private lastCallTime = 0;
  private minInterval = 1000; // 1 second between calls

  /**
   * Schedules a function to be executed with rate limiting
   * @param fn Function to execute
   * @returns Promise that resolves with the function's result
   */
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

  /**
   * Processes the queue of scheduled functions
   */
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
        console.error('Error in rate-limited task:', error);
      }
    }

    // Process next item in queue
    this.processQueue();
  }
}

// Export a singleton instance for use across the application
export const apiRateLimiter = new ApiRateLimiter();
