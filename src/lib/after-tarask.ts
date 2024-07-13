import type { ExtendedDict } from './types';

const prepositionPattern =
	/^(?:а[бд]?|б[ея]зь?|[дз]а|д?ля|дзеля|[нп]ад?|пр[аы]|празь?|у|церазь?)$/;

const isSingleVowel = (str: string) => str.match(/[аеёіоуыэюя]/g)?.length === 1;

export const endZSoftenAndNiaBiaz: ExtendedDict = [
	[
		/ бе(зь? \S+)/g,
		($0: string, $1: string) => (isSingleVowel($1) ? ' бя' + $1 : $0),
	],
	[
		/ не (\S+)/g,
		($0: string, $1: string) =>
			isSingleVowel($1) && !prepositionPattern.test($1) ? ' ня ' + $1 : $0,
	],
	[/ б[ея]з(?= і\S*[ая]ў|ну )/g, ' бязь'],
	[/( (?:пра|цера)?з)(?= і\S*[ая]ў|ну )/g, (_$0, $1) => $1 + 'ь'],
];
