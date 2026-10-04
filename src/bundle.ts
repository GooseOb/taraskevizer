/**
 * Single-file browser bundle entry point.
 *
 * Bundled by esbuild (see the `bundle` script) into `dist/bundle.js` as an
 * IIFE global (`taraskevizer`) with the WebAssembly binary inlined, so
 * browsers get a fully synchronous API: no `init()` call, no extra fetch.
 *
 * Excluded from the `tsc` package build; typechecked via `tsc --noEmit`.
 */
import { initSync } from './wasm/taraskevizer_wasm.js';
import wasmBytes from './wasm/taraskevizer_wasm_bg.wasm';

initSync({ module: wasmBytes });

export * from './index.js';
