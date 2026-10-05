/**
 * taraskevizer — Belarusian orthography converter (Narkamaŭka → Taraskievica).
 *
 * The conversion core is Rust compiled to WebAssembly; this module is a thin
 * typed layer over it: top-level pipelines, option types and the browser
 * helper for interactive HTML output.
 *
 * Conversion options are owned by TypeScript (see `./config.js`): the WASM
 * side only parses the resulting shape, it never manages the config object.
 *
 * The pipelines are synchronous, but the WebAssembly module needs one
 * asynchronous initialization first (the `node` entry does it on import):
 *
 * @example
 * ```js
 * import { init, tarask, TaraskConfig } from 'taraskevizer';
 *
 * await init();
 * tarask('планета'); // → 'плянэта'
 * tarask('планета', new TaraskConfig({ abc: 'latin' })); // → 'planeta'
 * tarask('планета', { abc: 'latin' }); // plain objects work too
 * ```
 */
import init, {
	alphabetic as alphabeticPipeline,
	phonetic as phoneticPipeline,
	splitIntoChunks as splitIntoChunksPipeline,
	tarask as taraskPipeline,
} from './wasm/taraskevizer_wasm.js';
import { htmlConfigOptions, TaraskConfig } from './config.js';
import type {
	TaraskAlphabet,
	TaraskJ,
	TaraskVariations,
	TaraskWrappers,
} from './config.js';

export type { TaraskAlphabet, TaraskJ, TaraskVariations, TaraskWrappers };
export { createInteractiveTags } from './html-tag-interactions.js';
export { htmlConfigOptions, init, TaraskConfig };

/** Convert academic orthography to classical (Taraskievica). */
export const tarask = (text: string, config?: Partial<TaraskConfig>): string =>
	taraskPipeline(text, config);

/** Alphabet-only conversion (no Taraskevization). */
export const alphabetic = (
	text: string,
	config?: Partial<TaraskConfig>
): string => alphabeticPipeline(text, config);

/** Phonetic conversion. */
export const phonetic = (
	text: string,
	config?: Partial<TaraskConfig>
): string => phoneticPipeline(text, config);

/**
 * Split text into worker-ready chunks for parallel conversion.
 *
 * Same boundaries the native CLI feeds to rayon: cuts after a spacing char
 * at/after `len / chunks` bytes, never inside `<…>` tags or after
 * apostrophe-likes, with no upper bound on chunk size. Returns owned strings
 * (not byte offsets) because Rust byte offsets don't map to JS UTF-16 indices.
 *
 * Convert each chunk (in `Worker`s, each with its own WASM instance) and
 * concatenate the results in order:
 *
 * ```js
 * const chunks = splitIntoChunks(bigText, navigator.hardwareConcurrency);
 * const out = (await Promise.all(chunks.map((c) => convertInWorker(c)))).join('');
 * ```
 */
export const splitIntoChunks = (text: string, chunks: number): string[] =>
	splitIntoChunksPipeline(text, chunks);
