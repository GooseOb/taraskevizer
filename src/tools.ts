import { letters, lettersUpperCase } from './letters';
import { Alphabet, ExtendedDict } from './types';

export const replaceWithDict = (text: string, dict: ExtendedDict = []) => {
	for (const [pattern, result] of dict)
		text = text.replace(
			pattern,
			//@ts-ignore
			result
		);

	return text;
};

export const convertAlphabet = (text: string, abc: Alphabet) =>
	replaceWithDict(replaceWithDict(text, letters[abc]), lettersUpperCase[abc]);
