type AnyFn = (...args: any[]) => any;
type DeepPartial<T> = T extends object
	? T extends AnyFn
		? T
		: {
				[P in keyof T]?: DeepPartial<T[P]>;
		  }
	: T;

type Alphabet = 0 | 1 | 2 | 3;
// cyrillic | latin | arabic | greek
type OptionJ = 0 | 1 | 2;
// never | random | always
type Variation = 0 | 1 | 2;
// no | first | all
export type NonHtmlOptions = {
	nodeColors: boolean;
	h: boolean;
	variations: Variation;
};
export type HtmlOptions = { g: boolean };
export type ReplaceWithDict = (text: string, dict?: ExtendedDict) => string;
export type ToTarask = (
	text: string,
	replaceWithDict: ReplaceWithDict,
	wordlist: Dict,
	softers: Dict,
	afterTarask: ExtendedDict
) => string;
export type TaraskOptionsStrict = {
	abc: Alphabet;
	j: OptionJ;
	html: boolean | HtmlOptions;
	nonHtml: boolean | NonHtmlOptions;
	OVERRIDE_toTarask?: ToTarask;
};
export type TaraskOptions = DeepPartial<TaraskOptionsStrict>;
export type Tarask = (text: string, options?: TaraskOptions) => string;
export type Dict<T = RegExp> = [T, string][];
export type ExtendedDict = [
	RegExp,
	string | ((...substrings: string[]) => string),
][];
export type AlphabetDependentDict = { [key in Alphabet]?: Dict };
