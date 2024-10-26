import { iaWords, phonetic } from '../dict';
import { endZSoftenAndNiaBiaz, mutatingStep, soften } from '../lib';

export const phonetize = mutatingStep(({ text }) =>
	endZSoftenAndNiaBiaz(phonetic(iaWords(soften(text))))
);
