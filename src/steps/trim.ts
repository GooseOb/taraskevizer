import { TaraskStep } from './types';

export const trim: TaraskStep = (options) => {
	options.text = ` ${options.text.trim()} `;
};
