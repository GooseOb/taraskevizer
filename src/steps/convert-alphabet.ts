import { mutatingStep, replaceWithDict, replaceWithDicts } from '../lib';

export const convertAlphabet = mutatingStep(
	({
		text,
		cfg: {
			abc: { upper, lower },
		},
	}) => replaceWithDicts(text, [lower, upper])
);

export const convertAlphabetLowerCase = mutatingStep(
	({
		text,
		cfg: {
			abc: { lower },
		},
	}) => replaceWithDict(text, lower)
);
