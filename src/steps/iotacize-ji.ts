import { iwords } from '../dict';
import { dictFrom, mutatingStep, replaceWithDict } from '../lib';

const iDict = dictFrom.raw([
	[/([аеёіоуыэюя] )і ў/, '$1й у'],
	[/([аеёіоуыэюя] )і /, '$1й '],
	[/([аеёіоуыэюя] ?)і/, '$1йі'],
	[` і(?=${iwords})`, ' йі'],
]);

export const iotacizeJi = mutatingStep(({ text }) =>
	replaceWithDict(text, iDict)
);
