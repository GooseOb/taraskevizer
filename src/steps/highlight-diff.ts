import type { TaraskStep, SplittedTextStorage } from './types';
import { highlightDiff, ansiColorWrappers } from '../lib/index';
import { alphabets } from '../dict/index';

/**
 * Uses {@link highlightDiff}
 */
export const highlightDiffStep =
	(highlight: (content: string) => string): TaraskStep<SplittedTextStorage> =>
	({
		cfg: {
			general: { abc },
		},
		storage: { textArr, origArr },
	}) => {
		highlightDiff(textArr, origArr, abc === alphabets.cyrillic, highlight);
	};

/**
 * Uses {@link highlightDiff}
 */
export const highlightDiffStepNonHtml: TaraskStep<SplittedTextStorage> = ({
	cfg: {
		general: { abc },
		nonHtml: { ansiColors },
	},
	storage: { textArr, origArr },
}) => {
	if (ansiColors)
		highlightDiff(
			textArr,
			origArr,
			abc === alphabets.cyrillic,
			ansiColorWrappers.fix
		);
};
