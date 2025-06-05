// miniquad_runtime.js is now loaded as a separate script in index.html,
// so we don't import it here. Its globals are expected to be available.

// Declare global variables exposed by miniquad_runtime.js and our custom set_wasm for TypeScript
declare global {
    var miniquad_add_plugin: (plugin: any) => void;
    var load: (wasmPath: string) => void;
    var wasm_exports: any; // This will hold the exports from your WASM module
    var set_wasm: (w: any) => void; // Declare our custom global set_wasm function
}

// Corrected path to the game WASM bindgen output
import init from "../../dist/game"; // No longer importing set_wasm directly from here

console.log("Hello from TypeScript!");

/**
 * Initializes and runs the miniquad game.
 * This function encapsulates the logic previously found in web/index.html for game loading.
 */
export async function runMiniquadGame() {
    const canvas = document.getElementById('glcanvas') as HTMLCanvasElement;
    if (!canvas) {
        console.error("Canvas element with ID 'glcanvas' not found.");
        return;
    }

    // Ensure canvas has focus and is ready for input
    canvas.focus();
    canvas.click(); // Trigger click to ensure it's active

    let wbg = await init();
    miniquad_add_plugin({
        register_plugin: (a: any) => (a.wbg = wbg),
        on_init: () => set_wasm(wasm_exports), // wasm_exports is a global from miniquad_runtime
        version: "0.1",
        name: "wbg",
    });
    // Corrected path to the WASM binary relative to the final bundle.js location in deploy/
    load("../game_bg.wasm");
}

// Expose runMiniquadGame globally so it can be called from index.html
(window as any).runMiniquadGame = runMiniquadGame;
