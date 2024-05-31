import { TaraskStep, SplittedTextStorage } from './types';

export const joinSplittedText: TaraskStep<SplittedTextStorage> = (
	_,
	{ storage: { text } }
) => text.join(' ');
