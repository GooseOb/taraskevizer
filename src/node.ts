/**
 * Node.js entry point: the same API as the default entry, with WebAssembly
 * initialized from the bundled binary on import — no `init()` call needed.
 *
 * ```js
 * import { tarask } from 'taraskevizer'; // `node` export condition
 * tarask('планета'); // → 'плянэта'
 * ```
 */
import { readFileSync } from 'node:fs';
import init from './wasm/taraskevizer_wasm.js';

const wasmUrl = new URL('./wasm/taraskevizer_wasm_bg.wasm', import.meta.url);
await init({ module_or_path: readFileSync(wasmUrl) });

export * from './index.js';
