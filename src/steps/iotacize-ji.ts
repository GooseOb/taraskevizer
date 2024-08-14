import { iwords } from '../dict';
import { dictFrom, mutatingStep, replaceWithDict } from '../lib';

const iDict = dictFrom.raw([
	[/([аеёіоуыэюя\u0301] )і ў/, '$1й у'],
	[/([аеёіоуыэюя\u0301] )і /, '$1й '],
	[/([аеёіоуыэюя\u0301] ?)і/, '$1йі'],
	[` і(?=${iwords})`, ' йі'],
]);

export const iotacizeJi = mutatingStep(({ text }) =>
	replaceWithDict(text, iDict)
);
