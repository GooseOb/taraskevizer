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
type Alphabet = 0 | 1 | 2 | 3;
type OptionJ = 0 | 1 | 2;
type Variation = 0 | 1 | 2;
export type TaraskOptions = {
	abc: Alphabet;
	j: OptionJ;
	OVERRIDE_toTarask?: ToTarask;
};
export type NonHtmlOptions = {
	ansiColors: boolean;
	h: boolean;
	variations: Variation;
};
export type HtmlOptions = {
	g: boolean;
};
export type ReplaceWithDict = (text: string, dict?: ExtendedDict) => string;
export type ToTarask = (
	text: string,
	replaceWithDict: ReplaceWithDict,
	wordlist: Dict,
	softers: Dict,
	afterTarask: ExtendedDict
) => string;
export type Tarask<TOptions extends object> = (
	text: string,
	taraskOptions?: DeepPartialReadonly<TaraskOptions>,
	options?: DeepPartialReadonly<TOptions>
) => string;
export type Dict<T = RegExp> = [T, string][];
export type ExtendedDict = [
	RegExp,
	string | ((...substrings: string[]) => string)
][];
export type AlphabetDependentDict = {
	[key in Alphabet]?: Dict;
};
export {};
