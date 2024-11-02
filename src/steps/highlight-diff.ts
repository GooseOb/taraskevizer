import type { TaraskStep, SplittedTextStorage } from './types';
import { highlightDiff } from '@/lib';
import { alphabets } from '@/dict';

/**
 * Uses {@link highlightDiff}
 */
export const highlightDiffStep: TaraskStep<SplittedTextStorage> = ({
	cfg: { abc, wrappers },
	storage: { textArr, origArr },
}) => {
	if (wrappers?.fix)
		highlightDiff(textArr, origArr, abc === alphabets.cyrillic, wrappers.fix);
};
