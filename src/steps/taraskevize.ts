import { wordlist } from '../dict';
import {
	endZSoftenAndNiaBiaz,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib';

export const taraskevize = mutatingStep(({ text }) =>
	replaceWithDict(soften(replaceWithDict(text, wordlist)), endZSoftenAndNiaBiaz)
);
