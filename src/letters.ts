import { ALPHABET } from './config-constants';
import {
	arabLetters,
	latinLettersLowerCase,
	latinLettersLowerCaseJi,
	latinLettersUpperCase,
	latinLettersUpperCaseJi,
} from './dict';
import { AlphabetDependentDict } from './types';

export const letters: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLettersLowerCase,
	[ALPHABET.ARABIC]: arabLetters,
	[ALPHABET.LATIN_JI]: latinLettersLowerCaseJi,
};
export const lettersUpperCase: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLettersUpperCase,
	[ALPHABET.LATIN_JI]: latinLettersUpperCaseJi,
};
