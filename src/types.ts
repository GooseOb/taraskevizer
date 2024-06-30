export type PartialReadonly<T> = {
	readonly [P in keyof T]?: T[P];
};

export type ValueOf<T> = T[keyof T];
