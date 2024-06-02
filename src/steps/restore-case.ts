import { restoreCase } from '../lib';
import type { TaraskStep, SplittedTextStorage } from './types';

export const restoreCaseStep: TaraskStep<SplittedTextStorage> = (
	_,
	{
		cfg: {
			general: { abc },
		},
		storage,
	}
) => {
	if (abc.upper) {
		storage.text = restoreCase(storage.text, storage.orig);
	}
	return _;
};
