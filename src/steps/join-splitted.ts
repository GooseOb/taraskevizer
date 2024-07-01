import { mutatingStep } from '../lib';
import { SplittedTextStorage } from './types';

export const joinSplittedText = mutatingStep<SplittedTextStorage>(
	({ storage: { textArr } }) => textArr.join(' ')
);
