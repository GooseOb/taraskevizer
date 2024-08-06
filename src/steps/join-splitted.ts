import { mutatingStep } from '../lib';
import type { SplittedTextStorage } from './types';

export const joinSplittedText = mutatingStep<SplittedTextStorage>(
	({ storage: { textArr } }) => textArr.join(' ')
);
