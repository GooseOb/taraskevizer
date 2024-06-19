import { noSoften, iaWords } from '../dict/index';
import {
	afterTarask,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib/index';

const noSoftenPlusIa = noSoften.concat(iaWords as any);

export const phonetize = mutatingStep(({ text }) =>
	replaceWithDict(
		soften(replaceWithDict(text, noSoftenPlusIa))
			.replace(/\ue0ff/g, '')
			.replace(/не пра/g, 'не&nbsp;пра'),
		afterTarask
	).replace(/не&nbsp;пра/g, 'не пра')
);
