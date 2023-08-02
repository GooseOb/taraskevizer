type Alphabet = 0 | 1 | 2;
type J = 0 | 1 | 2;
type HtmlOptions = {
	g: boolean;
};
type TaraskOptions = {
	abc: Alphabet;
	j: J;
	html: false | HtmlOptions;
};
type Promisify<T> = T extends (...args: infer TArgs) => infer TReturn
	? (...args: TArgs) => Promise<TReturn>
	: never;
type Tarask = (text: string, options: TaraskOptions) => string;
type TaraskAsync = Promisify<Tarask>;
type Dict = Record<string, RegExp>;
type AlphabetDependentDict = {
	[key in Alphabet]?: Dict;
};

declare const taraskSync: Tarask;
declare const tarask: TaraskAsync;

declare const gobj: {
	readonly г: 'ґ';
	readonly Г: 'Ґ';
	readonly ґ: 'г';
	readonly Ґ: 'Г';
};

export {
	AlphabetDependentDict,
	Dict,
	HtmlOptions,
	Tarask,
	TaraskAsync,
	TaraskOptions,
	gobj,
	tarask,
	taraskSync,
};
