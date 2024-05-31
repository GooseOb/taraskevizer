import type { TaraskStep, SplittedTextStorage } from './types';
import { highlightDiff, ansiColorWrappers } from '../lib';
import { ALPHABET } from '../alphabet';

export const highlightDiffStep =
	(highlight: (content: string) => string): TaraskStep<SplittedTextStorage> =>
	(
		_,
		{
			cfg: {
				general: { abc },
			},
			storage: { text, orig },
		}
	) => {
		highlightDiff(text, orig, abc === ALPHABET.CYRILLIC, highlight);
		return _;
	};

export const highlightDiffNonHtmlStep: TaraskStep<SplittedTextStorage> = (
	_,
	{
		cfg: {
			general: { abc },
			nonHtml: { ansiColors },
		},
		storage: { text, orig },
	}
) => {
	if (ansiColors)
		highlightDiff(text, orig, abc === ALPHABET.CYRILLIC, ansiColorWrappers.fix);
	return _;
};
