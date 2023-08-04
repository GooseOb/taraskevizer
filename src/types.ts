type Alphabet = 0 | 1 | 2;
type J = 0 | 1 | 2;
export type HtmlOptions = { g: boolean };
export type TaraskOptions = {
	abc: Alphabet;
	j: J;
	html: false | HtmlOptions;
};
type Promisify<T> = T extends (...args: infer TArgs) => infer TReturn
	? (...args: TArgs) => Promise<TReturn>
	: never;
export type Tarask = (text: string, options?: TaraskOptions) => string;
export type TaraskAsync = Promisify<Tarask>;
export type Dict = Record<string, RegExp>;
export type AlphabetDependentDict = { [key in Alphabet]?: Dict };
