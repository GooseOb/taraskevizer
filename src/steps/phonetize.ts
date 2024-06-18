import { iaWords } from '../dict/iawords';
import { noSoften, softeners } from '../dict/index';
import { iwords } from '../dict/iwords';
import {
	afterTarask,
	dictFrom,
	mutatingStep,
	replaceWithDict,
} from '../lib/index';

export const phonetize = mutatingStep(({ text }) => {
	text = replaceWithDict(text, noSoften.concat(iaWords as any));
	softening: do {
		text = replaceWithDict(text, softeners);
		for (const [pattern, result] of softeners)
			if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
		break;
	} while (true);

	return replaceWithDict(
		text.replace(/\ue0ff/g, '').replace(/не пра/g, 'не&nbsp;пра'),
		afterTarask
	).replace(/не&nbsp;пра/g, 'не пра');
});

const idict = dictFrom.raw([
	[/([аеёіоуыэюя] )і ў/, '$1й у'],
	[/([аеёіоуыэюя] )і /, '$1й '],
	[` і(?=${iwords})`, ' йі'],
]);
export const iotacizeJi = mutatingStep(({ text }) =>
	replaceWithDict(text, idict)
);
