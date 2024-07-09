export type PartialReadonly<T> = {
	readonly [P in keyof T]?: T[P];
};
