import { TaraskStep } from './types';

export const restoreParentheses: TaraskStep = (options) => {
	options.text = options.text.replace(/&#40/g, '(');
};

export const afterJoin: TaraskStep = (options) => {
	options.text = options.text
		.replace(/&nbsp;/g, ' ')
		.replace(/ (\p{P}|\p{S}|\d+|&#40) /gu, '$1');
};

export const finalize =
	(newLine: string): TaraskStep =>
	(options) => {
		options.text = options.text.replace(/\n/g, newLine).trim();
	};

export const toLowerCase: TaraskStep = (options) => {
	options.text = options.text.toLowerCase();
};
