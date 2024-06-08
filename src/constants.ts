export const REPLACE_J = {
	NEVER: 0,
	RANDOM: 1,
	ALWAYS: 2,
} as const;

export const VARIATION = {
	NO: 0,
	FIRST: 1,
	ALL: 2,
} as const;

type ValueOf<T> = T[keyof T];

export type OptionJ = ValueOf<typeof REPLACE_J>;

export type Variation = ValueOf<typeof VARIATION>;
