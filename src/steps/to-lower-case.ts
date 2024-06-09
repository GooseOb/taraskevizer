import { TaraskStep } from './types';

export const toLowerCase: TaraskStep = (options) => {
	options.text = options.text.toLowerCase();
};
