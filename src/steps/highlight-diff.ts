import type { TaraskStep, SplittedTextStorage } from './types';
import { highlightDiff } from '@/lib';
import { alphabets } from '@/dict';

/**
 * Uses {@link highlightDiff}
 */
export const highlightDiffStep: TaraskStep<SplittedTextStorage> = ({
	cfg: { abc, wrapperDict },
	storage: { textArr, origArr },
}) => {
	if (wrapperDict?.fix)
		highlightDiff(
			textArr,
			origArr,
			abc === alphabets.cyrillic,
			wrapperDict.fix
		);
};
