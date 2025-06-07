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
import { setupTonWalletIntegration } from './ton';
import { runMiniquadGame } from './game_loader';

/**
 * Main application entry point
 */
async function initializeApplication(): Promise<void> {
  initParticles();
  await setupTonWalletIntegration();
}

(window as any).runMiniquadGame = runMiniquadGame;

document.addEventListener('DOMContentLoaded', initializeApplication);
