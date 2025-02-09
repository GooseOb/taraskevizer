import { wordlist } from '@/dict';
import { endZSoftenAndNiaBiaz, mutatingStep, soften } from '@/lib';
import { dict } from '@/lib/debug';

export const taraskevize = mutatingStep(({ text }) =>
	endZSoftenAndNiaBiaz(soften(dict(wordlist, /меля/)(text)))
);
