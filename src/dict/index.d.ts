import type { Dict } from '../types';
import { latinLetters, latinLettersUpperCase } from './latin';
import { greekLetters, greekLettersUpperCase, thWords } from './greek';
declare const gobj: {
	readonly г: 'ґ';
	readonly Г: 'Ґ';
	readonly ґ: 'г';
	readonly Ґ: 'Г';
};
declare const wordlist: Dict, softers: Dict, arabLetters: Dict;
export {
	wordlist,
	softers,
	arabLetters,
	latinLetters,
	latinLettersUpperCase,
	greekLetters,
	greekLettersUpperCase,
	thWords,
	gobj,
};
