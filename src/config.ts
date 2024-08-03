import { alphabets } from './dict';
import type { PartialReadonly } from './types';
import { Alphabet } from './dict/alphabets';
import { WrapperDict, html } from './wrappers';

export type Variation = 'no' | 'first' | 'all';
export type OptionJ = 'never' | 'random' | 'always';

export class TaraskConfig {
	constructor(options?: PartialReadonly<TaraskConfig>) {
		for (const key in options) {
			const value = (options as any)[key];
			if (key in this && value !== undefined) (this as any)[key] = value;
		}
	}

	/**
	 * Predefined alphabets are in {@link dicts.alphabets}.
	 *
	 * @default alphabets.cyrillic
	 */
	abc = alphabets.cyrillic as Alphabet;

	/**
	 * | When to replace `і`(`i`) by `й`(`j`) after vowels | Example                  |
	 * | ------------------------------------------------- | ------------------------ |
	 * |                                                   | `яна і ён`               |
	 * | never                                             | `яна і ён`               |
	 * | random                                            | `яна і ён` or `яна й ён` |
	 * | always                                            | `яна й ён`               |
	 *
	 * Has no effect with abc set to {@link dicts.alphabets.latinJi}.
	 *
	 * @default "never"
	 */
	j = 'never' as OptionJ;

	/**
	 * If set to false, may cause unwanted changes in acronyms.
	 *
	 * @default true
	 */
	doEscapeCapitalized = true as boolean;

	/**
	 * Used for wrapping changed parts.
	 * Predefined dicts are in {@link wrappers}.
	 *
	 * If `null`, wrapping changes will be skipped.
	 *
	 * @default null
	 */
	wrapperDict = null as null | WrapperDict;

	/**
	 * Do replace ґ(g) by г(h) in cyrillic alphabet?
	 *
	 * | Value | Example                                 |
	 * | ----- | --------------------------------------- |
	 * | true  | Ґвалт ґвалт                             |
	 * | false | Гвалт гвалт                             |
	 * | true  | `<tarH>ґ</tarH>валт <tarH>Ґ</tarH>валт` |
	 * | false | `<tarH>г</tarH>валт <tarH>Г</tarH>валт` |
	 *
	 * @default false
	 */
	g = true as boolean;

	/**
	 * | Which variation is used if a part of word is variable | Example           |
	 * | ----------------------------------------------------- | ----------------- |
	 * |                                                       | Гродна            |
	 * | no (main)                                             | Гродна            |
	 * | first                                                 | Горадня           |
	 * | all                                                   | (Гродна\|Горадня) |
	 *
	 * @default "all"
	 */
	variations = 'all' as Variation;

	/**
	 * String to replace `"\n"` with.
	 *
	 * @default "\n"
	 * @example "<br>"
	 */
	newLine = '\n' as string;

	/**
	 * String to replace `"<"` with.
	 *
	 * @default "<"
	 * @example "&lt;"
	 */
	leftAngleBracket = '<' as string;
}

/**
 * Predefined configuration for HTML.
 *
 * @example
 * const htmlCfg = new TaraskConfig({
 *   ...myOptions,
 *   ...htmlConfigOptions
 * });
 */
export const htmlConfigOptions = {
	wrapperDict: html,
	g: false,
	newLine: '<br>',
	leftAngleBracket: '&lt',
} as const satisfies PartialReadonly<TaraskConfig>;
