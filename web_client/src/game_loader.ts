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
}

import init from '../../dist/game';

/**
 * Initializes and runs the Miniquad game
 */
export async function runMiniquadGame(): Promise<void> {
  const canvas = document.getElementById('glcanvas') as HTMLCanvasElement;
  if (!canvas) {
    console.error("Canvas element with ID 'glcanvas' not found.");
    return;
  }

  // Focus and click the canvas to ensure it captures input events
  canvas.focus();
  canvas.click();

  try {
    // Initialize the WebAssembly module
    const wbg = await init();

    // Register the plugin with Miniquad
    miniquad_add_plugin({
      register_plugin: (a: any) => (a.wbg = wbg),
      on_init: () => set_wasm(wasm_exports),
      version: '0.1',
      name: 'wbg',
    });

    // Load the game WASM file
    load('../game_bg.wasm');
  } catch (error) {
    console.error('Failed to initialize game:', error);
  }
}
