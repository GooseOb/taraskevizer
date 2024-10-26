import { iwords } from '@/dict';
import { callableDict, mutatingStep, re } from '@/lib';

const iDict = callableDict([
	[/([аеёіоуыэюя\u0301] )і ў/g, '$1й у'],
	[/([аеёіоуыэюя\u0301] )і /g, '$1й '],
	[/([аеёіоуыэюя\u0301] ?)і/g, '$1йі'],
	[re.g(` і(?=${iwords})`), ' йі'],
]);

export const iotacizeJi = mutatingStep(({ text }) => iDict(text));
