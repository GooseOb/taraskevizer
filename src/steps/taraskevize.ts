import { wordlist } from '../dict';
import { endZSoftenAndNiaBiaz, mutatingStep, soften } from '../lib';

export const taraskevize = mutatingStep(({ text }) =>
	endZSoftenAndNiaBiaz(soften(wordlist(text)))
);
