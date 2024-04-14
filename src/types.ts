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

export type Alphabet = 0 | 1 | 2;
// cyrillic | latin | arabic
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

export type Dict<T = RegExp> = readonly (readonly [T, string])[];
export type ExtendedDict = readonly (readonly [
	RegExp,
	string | ((...substrings: string[]) => string),
])[];
export type AlphabetDependentDict = { [key in Alphabet]?: Dict };
