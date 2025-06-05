declare global {
  var miniquad_add_plugin: (plugin: any) => void;
  var load: (wasmPath: string) => void;
  var wasm_exports: any;
  var set_wasm: (w: any) => void;
  var Telegram: any;
}

import { initParticles } from "./particles";
import { setupTonWalletIntegration } from "./ton_wallet";
import { runMiniquadGame } from "./game_loader"; // Import runMiniquadGame

(window as any).runMiniquadGame = runMiniquadGame;

document.addEventListener("DOMContentLoaded", () => {
  initParticles();
  setupTonWalletIntegration();
});
