import { iwords } from '../dict/index';
import { dictFrom, mutatingStep, replaceWithDict } from '../lib';

const iDict = dictFrom.raw([
	[/([аеёіоуыэюя] )і ў/, '$1й у'],
	[/([аеёіоуыэюя] )і /, '$1й '],
	[` і(?=${iwords})`, ' йі'],
]);

export const iotacizeJi = mutatingStep(({ text }) =>
	replaceWithDict(text, iDict)
);
