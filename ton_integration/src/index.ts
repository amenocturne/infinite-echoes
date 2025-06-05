declare global {
    var miniquad_add_plugin: (plugin: any) => void;
    var load: (wasmPath: string) => void;
    var wasm_exports: any; // This will hold the exports from your WASM module
    var set_wasm: (w: any) => void; // Declare our custom global set_wasm function
}

import init from "../../dist/game"; // No longer importing set_wasm directly from here
import { initParticles } from "./particles"; // Import initParticles

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
(window as any).runMiniquadGame = runMiniquadGame;

// Initialize particles when the script loads
document.addEventListener('DOMContentLoaded', () => {
    initParticles();
});
