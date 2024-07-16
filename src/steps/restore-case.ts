import { restoreCase } from '../lib';
import type { TaraskStep, SplittedTextStorage } from './types';

const isNotLowerCase = (str: string) => str !== str.toLowerCase();

export const restoreCaseStep: TaraskStep<SplittedTextStorage> = ({
	cfg: { abc },
	storage: { textArr, origArr },
}) => {
	if (abc.upper) {
		restoreCase(textArr, origArr);
	} else {
		for (let i = 0; i < textArr.length; i++) {
			if (isNotLowerCase(origArr[i])) textArr[i] = origArr[i];
		}
	}
};
