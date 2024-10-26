import { mutatingStep } from '../lib';
import * as debug from '../lib/debug';

export const convertAlphabet = mutatingStep(
	({
		text,
		cfg: {
			abc: { upper, lower },
		},
	}) => {
		text = lower(text);
		return upper ? debug.dict(upper, /K/)(text) : text;
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
