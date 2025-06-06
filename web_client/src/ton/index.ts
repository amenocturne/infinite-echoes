import { tonService } from './services/ton_service';
import { tonUIService } from './ui/ton_ui_service';
import { tonBridgeService } from './bridge/ton_bridge';

/**
 * Sets up the TON wallet integration
 */
export async function setupTonWalletIntegration(): Promise<void> {
  try {
    // Initialize the bridge
    tonBridgeService.initialize();

    // Initialize the UI service
    tonUIService.initialize();

    // Initialize the TON service
    await tonService.initialize();
  } catch (error) {
    console.error('Error setting up TON wallet integration:', error);
  }
}

// Re-export services for direct access
export * from './services';
export * from './ui';
export * from './bridge';
