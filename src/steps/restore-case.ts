import { restoreCase } from '../lib';
import type { TaraskStep, SplittedTextStorage } from './types';

export const restoreCaseStep: TaraskStep<SplittedTextStorage> = (
	_,
	{
		cfg: {
			general: { abc },
		},
		storage: { text, orig },
	}
) => {
	if (abc.upper) {
		restoreCase(text, orig);
	} else {
		for (let i = 0; i < text.length; i++) {
			if (orig[i] !== orig[i].toLowerCase()) text[i] = orig[i];
		}
	}
	return _;
};
