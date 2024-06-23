import type { ExtendedDict } from './types';

const prepositionPattern =
	/^(?:а[бд]?|б[ея]зь?|да|для|дзеля|зь?|за|ля|над?|пад?|пра|празь?|пры|у|церазь?)$/;

const isSingleVowel = (str: string) => str.match(/[аеёіоуыэюя]/g)?.length === 1;

export const afterTarask: ExtendedDict = [
	[
		/ бе(зь? \S+)/g,
		($0: string, $1: string) => (isSingleVowel($1) ? ' бя' + $1 : $0),
	],
	[
		/ не (\S+)/g,
		($0: string, $1: string) =>
			isSingleVowel($1) && !prepositionPattern.test($1) ? ' ня ' + $1 : $0,
	],
	[
		/( (?:б[ея]|пра|цера)?з) і(\S*)/g,
		($0, $1, $2) => (/([ая]ў|ну)$/.test($2) ? $1 + 'ь і' + $2 : $0),
	],
];
