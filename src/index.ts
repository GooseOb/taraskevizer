/**
 * taraskevizer — Belarusian orthography converter (Narkamaŭka → Taraskievica).
 *
 * The conversion core is Rust compiled to WebAssembly; this module is a thin
 * typed layer over it: top-level pipelines, option types and the browser
 * helper for interactive HTML output.
 *
 * The pipelines are synchronous, but the WebAssembly module needs one
 * asynchronous initialization first (the `node` entry does it on import):
 *
 * @example
 * ```js
 * import init, { tarask, TaraskConfig } from 'taraskevizer';
 *
 * await init();
 * tarask('планета'); // → 'плянэта'
 * tarask('планета', new TaraskConfig({ abc: 'latin' })); // → 'planeta'
 * tarask('планета', { abc: 'latin' }); // plain objects work too
 * ```
 */
import init, {
	alphabetic as alphabeticPipeline,
	htmlConfigOptions as htmlConfigOptionsBase,
	phonetic as phoneticPipeline,
	tarask as taraskPipeline,
	TaraskConfig as TaraskConfigBase,
} from './wasm/taraskevizer_wasm.js';
import type {
	TaraskAlphabet,
	TaraskJ,
	TaraskOptions,
	TaraskVariations,
	TaraskWrappers,
} from './wasm/taraskevizer_wasm.js';

export type {
	TaraskAlphabet,
	TaraskJ,
	TaraskOptions,
	TaraskVariations,
	TaraskWrappers,
};
export { createInteractiveTags } from './html-tag-interactions.js';
export { init };

/** Config accepted by the pipelines: an instance, a plain object, or omitted. */
export type ConfigLike = TaraskConfig | TaraskOptions | null | undefined;

/**
 * Conversion options. Mirrors the previous `TaraskConfig` API: every field
 * keeps its name (`doEscapeCapitalized`, `newLine`, …) and defaults.
 * Only predefined wrapper sets are available (`wrappers: 'none' | 'html' |
 * 'ansi'`); arbitrary callback wrappers cannot cross the WASM boundary.
 */
export class TaraskConfig extends TaraskConfigBase {
	constructor(options?: TaraskOptions) {
		super(options);
	}
}

/** Predefined configuration for HTML output (wrappers + `<br>` newlines). */
export const htmlConfigOptions = (): TaraskConfig =>
	htmlConfigOptionsBase() as TaraskConfig;

/** Convert academic orthography to classical (Taraskievica). */
export const tarask = (text: string, config?: ConfigLike): string =>
	taraskPipeline(text, config);

/** Alphabet-only conversion (no Taraskevization). */
export const alphabetic = (text: string, config?: ConfigLike): string =>
	alphabeticPipeline(text, config);

/** Phonetic conversion. */
export const phonetic = (text: string, config?: ConfigLike): string =>
	phoneticPipeline(text, config);
