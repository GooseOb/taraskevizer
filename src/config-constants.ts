import { Alphabet, OptionJ, Variation } from './types';

export const ALPHABET = {
	CYRILLIC: 0,
	LATIN: 1,
	ARABIC: 2,
	LATIN_JI: 3,
} as const satisfies Record<string, Alphabet>;
export const REPLACE_J = {
	NEVER: 0,
	RANDOM: 1,
	ALWAYS: 2,
} as const satisfies Record<string, OptionJ>;
export const VARIATION = {
	NO: 0,
	FIRST: 1,
	ALL: 2,
} as const satisfies Record<string, Variation>;
