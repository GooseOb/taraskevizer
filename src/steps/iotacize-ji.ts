import { iwords } from '../dict';
import { regexG } from '../dict/lib';
import { dict, mutatingStep } from '../lib';

const iDict = dict([
	[/([аеёіоуыэюя\u0301] )і ў/g, '$1й у'],
	[/([аеёіоуыэюя\u0301] )і /g, '$1й '],
	[/([аеёіоуыэюя\u0301] ?)і/g, '$1йі'],
	[regexG(` і(?=${iwords})`), ' йі'],
]);

export const iotacizeJi = mutatingStep(({ text }) => iDict(text));
