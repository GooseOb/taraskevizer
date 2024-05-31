import { replaceWithDict } from '../lib';
import type { TaraskStep } from './types';

export const convertAlphabet: TaraskStep = (
	text,
	{
		cfg: {
			general: { abc },
		},
	}
) => replaceWithDict(replaceWithDict(text, abc.lower), abc.upper);

export const convertAlphabetLowerCase: TaraskStep = (
	text,
	{
		cfg: {
			general: { abc },
		},
	}
) => replaceWithDict(text, abc.lower);
