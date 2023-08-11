type Promisify<T> = T extends (...args: infer TArgs) => infer TReturn
	? (...args: TArgs) => Promise<TReturn>
	: never;
type DeepPartial<T> = T extends object
	? {
			[P in keyof T]?: DeepPartial<T[P]>;
	  }
	: T;

type Alphabet = 0 | 1 | 2;
type J = 0 | 1 | 2;
export type HtmlOptions = { g: boolean };
export type TaraskOptions = {
	abc: Alphabet;
	j: J;
	html: false | HtmlOptions;
};
export type Tarask = (
	text: string,
	options?: DeepPartial<TaraskOptions>
) => string;
export type TaraskAsync = Promisify<Tarask>;
export type Dict<T = RegExp> = [string, T][];
export type AlphabetDependentDict = { [key in Alphabet]?: Dict };
