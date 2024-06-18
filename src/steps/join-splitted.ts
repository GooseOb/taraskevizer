import { mutatingStep } from '../lib/index';
import { SplittedTextStorage } from './types';

export const joinSplittedText = mutatingStep<SplittedTextStorage>(
	({ storage: { textArr } }) => textArr.join(' ')
);
