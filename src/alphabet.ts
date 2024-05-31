import {
	arabLetters,
	latinLettersLowerCase,
	latinLettersLowerCaseJi,
	latinLettersUpperCase,
	latinLettersUpperCaseJi,
} from './dict';
import { Alphabet } from './types';

export const ALPHABET = {
	LATIN: {
		lower: latinLettersLowerCase,
		upper: latinLettersUpperCase,
	},

	LATIN_JI: {
		lower: latinLettersLowerCaseJi,
		upper: latinLettersUpperCaseJi,
	},

	ARABIC: {
		lower: arabLetters,
	},

	CYRILLIC: {
		lower: [],
		upper: [],
	},
} satisfies Record<string, Alphabet>;
