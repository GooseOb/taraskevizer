import { wordlist } from '../dict/index';
import {
	endZSoftenAndNiaBiaz,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib/index';

export const taraskevize = mutatingStep(({ text }) =>
	replaceWithDict(soften(replaceWithDict(text, wordlist)), endZSoftenAndNiaBiaz)
);
