import { iaWords, phonetic } from '../dict';
import {
	endZSoftenAndNiaBiaz,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib';

const dict = iaWords.concat(phonetic);

export const phonetize = mutatingStep(({ text }) =>
	replaceWithDict(soften(replaceWithDict(text, dict)), endZSoftenAndNiaBiaz)
);
