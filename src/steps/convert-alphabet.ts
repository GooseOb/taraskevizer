import { replaceWithDict } from '../lib';
import type { TaraskStep } from './types';

export const convertAlphabet: TaraskStep = (options) => {
	const { lower, upper } = options.cfg.general.abc;
	options.text = replaceWithDict(replaceWithDict(options.text, lower), upper);
};

export const convertAlphabetLowerCase: TaraskStep = (options) => {
	options.text = replaceWithDict(options.text, options.cfg.general.abc.lower);
};
