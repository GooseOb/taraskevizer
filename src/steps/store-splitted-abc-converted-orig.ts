import type { TaraskStep, SplittedTextStorage } from './types';

export const storeSplittedAbcConvertedOrig: TaraskStep<SplittedTextStorage> = ({
	cfg: {
		abc: { lower, upper },
	},
	text,
	storage,
}) => {
	text = lower(text);
	storage.origArr = (upper ? upper(text) : text).split(' ');
};
