import { iaWords, phonetic } from '../dict/index';
import {
	endZSoftenAndNiaBiaz,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib/index';

const dict = iaWords.concat(phonetic);

export const phonetize = mutatingStep(({ text }) =>
	replaceWithDict(soften(replaceWithDict(text, dict)), endZSoftenAndNiaBiaz)
);
