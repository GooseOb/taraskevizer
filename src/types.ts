import type { Alphabet } from './dict/alphabets';
import type { OptionJ, Variation } from './constants';

export type PartialReadonly<T> = {
	readonly [P in keyof T]?: T[P];
};

export type TaraskOptions = {
	/**
	 * Any object that implements the {@link Alphabet} interface.
	 *
	 * The set of defined alphabets can be found in {@link dicts.alphabets}.
	 *
	 * @default alphabets.cyrillic
	 */
	abc: Alphabet;
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
	j: OptionJ;
	/**
	 * If set to false, may cause unwanted changes in acronyms.
	 *
	 * @default true
	 */
	doEscapeCapitalized: boolean;
};
export type NonHtmlOptions = {
	/** @default false */
	ansiColors: boolean;
	/**
	 * Do replace ґ(g) by г(h) in cyrillic alphabet?
	 *
	 * | Value | Example     |
	 * | ----- | ----------- |
	 * | true  | Ґвалт ґвалт |
	 * | false | Гвалт гвалт |
	 *
	 * @default false
	 */
	h: boolean;
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
	variations: Variation;
};
export type HtmlOptions = {
	/**
	 * Do replace `г`(`h`) by `ґ`(`g`) in cyrillic alphabet?
	 *
	 * | Value | Example                                 |
	 * | ----- | --------------------------------------- |
	 * | true  | `<tarH>г</tarH>валт <tarH>Г</tarH>валт` |
	 * | false | `<tarH>ґ</tarH>валт <tarH>Ґ</tarH>валт` |
	 *
	 * @default false
	 */
	g: boolean;
};
