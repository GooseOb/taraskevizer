/**
 * Conversion options for the taraskevizer pipelines.
 *
 * This module owns the `TaraskConfig` class: a plain TypeScript class.
 * The Rust/WASM side only parses the resulting shape (a `TaraskConfig`
 * instance or a plain `TaraskOptions` object) via `Reflect`, it never
 * manages the config object itself.
 *
 * No runtime validation here — the types are the contract. Invalid values
 * from untyped callers are rejected by the Rust parser when a pipeline runs.
 */

export type TaraskAlphabet = 'cyrillic' | 'latin' | 'latinJi' | 'arabic';
export type TaraskJ = 'never' | 'random' | 'always';
export type TaraskVariations = 'all' | 'no' | 'first';
export type TaraskWrappers = 'none' | 'html' | 'ansi';

/**
 * Conversion options. Every field keeps its name (`doEscapeCapitalized`,
 * `newLine`, …) and default. Only predefined wrapper sets are available
 * (`wrappers: 'none' | 'html' | 'ansi'`).
 *
 * ```js
 * new TaraskConfig() // defaults
 * new TaraskConfig({ abc: 'latin', g: false })
 * ```
 */
export class TaraskConfig {
	/** Alphabet: `"cyrillic"` (default), `"latin"`, `"latinJi"` or `"arabic"`. */
	abc: TaraskAlphabet = 'cyrillic';
	/**
	 * When to replace `і` by `й` after vowels: `"never"` (default),
	 * `"random"` or `"always"`.
	 */
	j: TaraskJ = 'never';
	/** Whether capitalized words are protected from changes (default `true`). */
	doEscapeCapitalized: boolean = true;
	/** Active wrapper set: `"none"` (default), `"html"` or `"ansi"`. */
	wrappers: TaraskWrappers = 'none';
	/**
	 * Whether to convert `ґ→г`-style `г` into `ґ` where appropriate
	 * (default `true`; `false` in `htmlConfigOptions()`).
	 */
	g: boolean = true;
	/** Which word variation to keep: `"all"` (default), `"no"` or `"first"`. */
	variations: TaraskVariations = 'all';
	/** Replacement for `"\n"` (default `"\n"`, `"<br>"` in HTML mode). */
	newLine: string = '\n';
	/** Replacement for `"<"` (default `"<"`, `"&lt"` in HTML mode). */
	leftAngleBracket: string = '<';
	/** Placeholder for `<…>`-protected spans (default `" \uE0FE "`). */
	noFixPlaceholder: string = ' \uE0FE ';

	constructor(options?: Partial<TaraskConfig> | null) {
		Object.assign(this, options ?? {});
	}
}

/**
 * Predefined configuration for HTML output: HTML wrappers, no `ґ→г`
 * conversion, `"<br>"` newlines and `"&lt"` for `"<"`.
 */
export const htmlConfigOptions = (): TaraskConfig =>
	new TaraskConfig({
		wrappers: 'html',
		g: false,
		newLine: '<br>',
		leftAngleBracket: '&lt',
	});
