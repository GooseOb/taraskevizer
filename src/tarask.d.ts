import type { Tarask, NonHtmlOptions, HtmlOptions } from './types';
export declare const ALPHABET: {
	readonly CYRILLIC: 0;
	readonly LATIN: 1;
	readonly ARABIC: 2;
	readonly GREEK: 3;
};
export declare const J: {
	readonly NEVER: 0;
	readonly RANDOM: 1;
	readonly ALWAYS: 2;
};
export declare const VARIATION: {
	readonly NO: 0;
	readonly FIRST: 1;
	readonly ALL: 2;
};
export declare const taraskToHtml: Tarask<HtmlOptions>;
export declare const tarask: Tarask<NonHtmlOptions>;
