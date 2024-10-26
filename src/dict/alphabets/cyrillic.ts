import { callableDict } from '../lib';
import type { Alphabet } from './types';

export const cyrillic = {
	lower: callableDict([]),
	upper: callableDict([]),
} as const satisfies Alphabet;
