declare global {
  var miniquad_add_plugin: (plugin: any) => void;
  var load: (wasmPath: string) => void;
  var wasm_exports: any;
  var set_wasm: (w: any) => void;
}

import init from "../../dist/game";

export async function runMiniquadGame() {
  const canvas = document.getElementById("glcanvas") as HTMLCanvasElement;
  if (!canvas) {
    console.error("Canvas element with ID 'glcanvas' not found.");
    return;
  }
  canvas.focus();
  canvas.click();

  let wbg = await init();
  miniquad_add_plugin({
    register_plugin: (a: any) => (a.wbg = wbg),
    on_init: () => set_wasm(wasm_exports),
    version: "0.1",
    name: "wbg",
  });
  load("../game_bg.wasm");
}
