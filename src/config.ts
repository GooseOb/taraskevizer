import { alphabets } from './dict';
import type { PartialReadonly, ValueOf } from './types';
import { REPLACE_J, VARIATION } from './constants';
import { Alphabet } from './dict/alphabets';
import { WrapperDict, htmlWrappers } from './wrappers';

export class TaraskConfig {
	constructor(options?: PartialReadonly<TaraskConfig>) {
		for (const key in options) {
			const value = (options as any)[key];
			if (key in this && value !== undefined) (this as any)[key] = value;
		}
	}

	/**
	 * Any object that implements the {@link Alphabet} interface.
	 *
	 * The set of defined alphabets can be found in {@link dicts.alphabets}.
	 *
	 * @default alphabets.cyrillic
	 */
	abc = alphabets.cyrillic as Alphabet;

	/**
	 * | Value | When to replace `і`(`i`) by `й`(`j`) after vowels | Example                  |
	 * | ----- | ------------------------------------------------- | ------------------------ |
	 * |       |                                                   | `яна і ён`               |
	 * | 0     | never                                             | `яна і ён`               |
	 * | 1     | random                                            | `яна і ён` or `яна й ён` |
	 * | 2     | always                                            | `яна й ён`               |
	 *
	 * Has no effect with abc set to {@link dicts.alphabets.latinJi}.
	 *
	 * @default REPLACE_J.NEVER
	 */
	j = REPLACE_J.NEVER as ValueOf<typeof REPLACE_J>;

	/**
	 * If set to false, may cause unwanted changes in acronyms.
	 *
	 * @default true
	 */
	doEscapeCapitalized = true as boolean;

	/** @default null */
	wrapperDict = null as null | WrapperDict;

	/**
	 * Do replace ґ(g) by г(h) in cyrillic alphabet?
	 *
	 * | Value | Example     |
	 * | ----- | ----------- |
	 * | true  | Ґвалт ґвалт |
	 * | false | Гвалт гвалт |
	 * | true  | `<tarH>ґ</tarH>валт <tarH>Ґ</tarH>валт` |
	 * | false | `<tarH>г</tarH>валт <tarH>Г</tarH>валт` |
	 *
	 * @default false
	 */
	g = true as boolean;

	/**
	 * | Value | Which variation is used if a part of word is variable | Example           |
	 * | ----- | ----------------------------------------------------- | ----------------- |
	 * |       |                                                       | Гродна            |
	 * | 0     | main                                                  | Гродна            |
	 * | 1     | first                                                 | Горадня           |
	 * | 2     | all                                                   | (Гродна\|Горадня) |
	 *
	 * @default VARIATION.ALL
	 */
	variations = VARIATION.ALL as ValueOf<typeof VARIATION>;

	newLine = '\n' as string;

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
	wrapperDict: htmlWrappers,
	g: false,
	newLine: '<br>',
	leftAngleBracket: '&lt',
} satisfies PartialReadonly<TaraskConfig>;
