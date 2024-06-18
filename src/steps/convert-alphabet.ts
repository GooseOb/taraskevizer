import { mutatingStep, replaceWithDict } from '../lib/index';

export const convertAlphabet = mutatingStep(
	({
		text,
		cfg: {
			general: {
				abc: { upper, lower },
			},
		},
	}) => replaceWithDict(replaceWithDict(text, lower), upper)
);

export const convertAlphabetLowerCase = mutatingStep(
	({
		text,
		cfg: {
			general: {
				abc: { lower },
			},
		},
	}) => replaceWithDict(text, lower)
);
