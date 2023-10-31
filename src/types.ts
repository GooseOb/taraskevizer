type ModifyObjectType<T, TResultObj> = T extends object
	? T extends (...args: any[]) => any
		? T
		: TResultObj
	: T;
// type DeepPartial<T> = ModifyObjectType<
// 	T,
// 	{
// 		[P in keyof T]?: DeepPartial<T[P]>;
// 	}
// >;
// type DeepReadonly<T> = ModifyObjectType<
// 	T,
// 	{
// 		readonly [P in keyof T]: DeepReadonly<T[P]>;
// 	}
// >;
export type DeepPartialReadonly<T> = ModifyObjectType<
	T,
	{
		readonly [P in keyof T]?: DeepPartialReadonly<T[P]>;
	}
>;

type Alphabet = 0 | 1 | 2 | 3;
// cyrillic | latin | arabic | greek
type OptionJ = 0 | 1 | 2;
// never | random | always
type Variation = 0 | 1 | 2;
// no | first | all
export type TaraskOptions = {
	abc: Alphabet;
	j: OptionJ;
	OVERRIDE_toTarask?: ToTarask;
};
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
export type AlphabetDependentDict = { [key in Alphabet]?: Dict };
