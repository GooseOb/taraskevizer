import type { Dict } from './dict/types';

type ModifyObjectType<T, TResultObj> = T extends object
	? T extends (...args: any[]) => any
		? T
		: TResultObj
	: T;
export type DeepPartialReadonly<T> = ModifyObjectType<
	T,
	{
		readonly [P in keyof T]?: DeepPartialReadonly<T[P]>;
	}
>;

export type Alphabet = {
	lower: Dict;
	upper?: Dict;
};
export type OptionJ = 0 | 1 | 2;
// never | random | always
export type Variation = 0 | 1 | 2;
// no | first | all

export type TaraskOptions = {
	doEscapeCapitalized: boolean;
	abc: Alphabet;
	j: OptionJ;
};
export type NonHtmlOptions = {
	ansiColors: boolean;
	h: boolean;
	variations: Variation;
};
export type HtmlOptions = { g: boolean };

export type ExtendedDict = readonly (readonly [
	RegExp,
	string | ((...substrings: string[]) => string),
])[];
