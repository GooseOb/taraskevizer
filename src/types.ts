export type PartialReadonly<T> = {
	readonly [P in keyof T]?: T[P];
};

type AnyFn = (...args: any[]) => any;

export type DeepReadonly<T> = T extends (infer R)[]
	? ReadonlyArray<DeepReadonly<R>>
	: T extends AnyFn
		? T
		: T extends object
			? {
					readonly [P in keyof T]: DeepReadonly<T[P]>;
				}
			: T;
export type DeepPartialReadonly<T> = T extends (infer R)[]
	? ReadonlyArray<DeepPartialReadonly<R>>
	: T extends AnyFn
		? T
		: T extends object
			? {
					readonly [P in keyof T]?: DeepPartialReadonly<T[P]>;
				}
			: T;
