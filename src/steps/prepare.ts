import type { TaraskStep } from './types';

/**
 * Prepares the text for correct processing.
 *
 * Some changes should be reverted in the {@link finalize} step.
 */
export const prepare: TaraskStep = (options) => {
	options.text = options.text
		.replace(/г'(?![еёіюя])/g, 'ґ')
		.replace(/ - /g, ' — ')
		.replace(/(\p{P}|\p{S}|\d+)/gu, ' $1 ')
		.replace(/ ['`’] (?=\S)/g, 'ʼ')
		.replace(/\(/g, '&#40');
};
