import { iaWords, phonetic } from '../dict';
import {
	endZSoftenAndNiaBiaz,
	mutatingStep,
	replaceWithDicts,
	soften,
} from '../lib';

export const phonetize = mutatingStep(({ text }) =>
	replaceWithDicts(soften(text), [iaWords, phonetic, endZSoftenAndNiaBiaz])
);
