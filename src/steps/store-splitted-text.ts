import type { TaraskStep, SplittedTextStorage } from './types';

export const storeSplittedText: TaraskStep<SplittedTextStorage> = ({
	text,
	storage,
}) => {
	storage.textArr = text.split(' ');
	if (storage.textArr.length !== storage.origArr.length)
		throw new Error(
			'The number of words in the source string is not equal to the processed one. Please, report the issue.'
		);
};
