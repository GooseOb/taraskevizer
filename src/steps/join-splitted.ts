import { TaraskStep, SplittedTextStorage } from './types';

export const joinSplittedText: TaraskStep<SplittedTextStorage> = (options) => {
	options.text = options.storage.textArr.join(' ');
};
