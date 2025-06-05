declare global {
  var miniquad_add_plugin: (plugin: {
    register_plugin: (a: any) => void;
    on_init: () => void;
    version: string;
    name: string;
  }) => void;
  var load: (wasmPath: string) => void;
  var wasm_exports: any;
  var set_wasm: (w: any) => void;
  var Telegram: any;
}

import { initParticles } from './particles';
import { setupTonWalletIntegration } from './ton/ton_wallet';
import { runMiniquadGame } from './game_loader';

/**
 * Main application entry point
 */
function initializeApplication(): void {
  // Initialize background particles
  initParticles();

  // Set up TON wallet integration
  setupTonWalletIntegration();
}

// Expose the game loader function globally
(window as any).runMiniquadGame = runMiniquadGame;

// Initialize the application when the DOM is fully loaded
document.addEventListener('DOMContentLoaded', initializeApplication);
