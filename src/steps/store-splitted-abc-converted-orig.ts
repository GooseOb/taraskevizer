import { convertAlphabet } from './convert-alphabet';
import type { TaraskStep, SplittedTextStorage } from './types';

export const storeSplittedAbcConvertedOrig: TaraskStep<SplittedTextStorage> = (
	text,
	options
) => {
	options.storage.orig = convertAlphabet(text, options).split(' ');
	return text;
};
