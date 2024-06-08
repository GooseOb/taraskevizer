import type { TaraskStep, SplittedTextStorage } from './types';

export const storeSplittedText: TaraskStep<SplittedTextStorage> = ({
	text,
	storage,
}) => {
	storage.textArr = text.split(' ');
};
