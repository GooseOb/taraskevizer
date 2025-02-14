import type { SplittedTextStorage, TaraskStep } from './types';

export const escapeLeftAngleBracket: TaraskStep<SplittedTextStorage> = ({
	cfg: { leftAngleBracket },
	storage: { textArr },
}) => {
	if (leftAngleBracket !== '<')
		for (let i = 0; i < textArr.length; i++)
			if (textArr[i] === '<') textArr[i] = leftAngleBracket;
};
