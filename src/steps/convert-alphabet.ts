import { mutatingStep } from '@/lib';

export const convertAlphabet = mutatingStep(
	({
		text,
		cfg: {
			abc: { upper, lower },
		},
	}) => {
		text = lower(text);
		return upper ? upper(text) : text;
	}
);

export const convertAlphabetLowerCase = mutatingStep(
	({
		text,
		cfg: {
			abc: { lower },
		},
	}) => lower(text)
);
