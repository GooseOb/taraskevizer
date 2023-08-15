type Promisify<T> = T extends (...args: infer TArgs) => infer TReturn
	? (...args: TArgs) => Promise<TReturn>
	: never;
type DeepPartial<T> = T extends object
	? {
			[P in keyof T]?: DeepPartial<T[P]>;
	  }
	: T;

type Alphabet = 0 | 1 | 2 | 3;
// cyrillic | latin | arabic | greek
type J = 0 | 1 | 2;
// never | random | always
type Variation = 0 | 1 | 2;
// no | first | all
export type NonHtmlOptions = {
	nodeColors: boolean;
	h: boolean;
	variations: Variation;
};
export type HtmlOptions = { g: boolean };
export type TaraskOptionsStrict = {
	abc: Alphabet;
	j: J;
} & (
	| {
			html: HtmlOptions;
			nonHtml: false;
	  }
	| {
			html: false;
			nonHtml: NonHtmlOptions;
	  }
);
export type TaraskOptions = DeepPartial<TaraskOptionsStrict>;
export type Tarask = (text: string, options?: TaraskOptions) => string;
export type TaraskAsync = Promisify<Tarask>;
export type Dict<T = RegExp> = [string, T][];
export type AlphabetDependentDict = { [key in Alphabet]?: Dict };
