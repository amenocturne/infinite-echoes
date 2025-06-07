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

  canvas.focus();
  canvas.click();

  try {
    const wbg = await init();

    miniquad_add_plugin({
      register_plugin: (a: any) => (a.wbg = wbg),
      on_init: () => set_wasm(wasm_exports),
      version: '0.1',
      name: 'wbg',
    });

    load('../game_bg.wasm');
  } catch (error) {
    console.error('Failed to initialize game:', error);
  }
}
