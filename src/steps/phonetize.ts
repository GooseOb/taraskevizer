import { noSoften, iaWords } from '../dict/index';
import {
	afterTarask,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib/index';
import { phonetic } from '../dict';

const dict = noSoften.concat(iaWords, phonetic);

export const phonetize = mutatingStep(({ text }) =>
	replaceWithDict(
		soften(replaceWithDict(text, dict)).replace(/\ue0ff/g, ''),
		afterTarask
	)
);
