import { iwords } from '../dict/index';
import { dictFrom, mutatingStep, replaceWithDict } from '../lib/index';

const iDict = dictFrom.raw([
	[/([аеёіоуыэюя] )і ў/, '$1й у'],
	[/([аеёіоуыэюя] )і /, '$1й '],
	[/([аеёіоуыэюя] ?)і/, '$1йі'],
	[` і(?=${iwords})`, ' йі'],
]);

export const iotacizeJi = mutatingStep(({ text }) =>
	replaceWithDict(text, iDict)
);
