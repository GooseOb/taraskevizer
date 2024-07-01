import { replaceWithDict } from '../lib';
import type { TaraskStep, SplittedTextStorage } from './types';

export const storeSplittedAbcConvertedOrig: TaraskStep<SplittedTextStorage> = ({
	cfg: {
		abc: { lower, upper },
	},
	text,
	storage,
}) => {
	storage.origArr = replaceWithDict(replaceWithDict(text, lower), upper).split(
		' '
	);
};
