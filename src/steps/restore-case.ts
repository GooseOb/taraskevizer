import { restoreCase } from '../lib';
import type { TaraskStep, SplittedTextStorage } from './types';

export const restoreCaseStep: TaraskStep<SplittedTextStorage> = ({
	cfg: {
		general: { abc },
	},
	storage: { textArr, origArr },
}) => {
	if (abc.upper) {
		restoreCase(textArr, origArr);
	} else {
		for (let i = 0; i < textArr.length; i++) {
			if (origArr[i] !== origArr[i].toLowerCase()) textArr[i] = origArr[i];
		}
	}
};
