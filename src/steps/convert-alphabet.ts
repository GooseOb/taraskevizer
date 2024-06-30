import { mutatingStep, replaceWithDict } from '../lib/index';

export const convertAlphabet = mutatingStep(
	({
		text,
		cfg: {
			abc: { upper, lower },
		},
	}) => replaceWithDict(replaceWithDict(text, lower), upper)
);

export const convertAlphabetLowerCase = mutatingStep(
	({
		text,
		cfg: {
			abc: { lower },
		},
	}) => replaceWithDict(text, lower)
);
