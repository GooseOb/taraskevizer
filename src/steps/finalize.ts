import { mutatingStep } from '../lib/index';
import type { TaraskStep } from './types';

/**
 * @param newLine - The string to replace new lines with.
 *
 * Reverse the changes made in the {@link prepare} step
 * and replace new lines with the passed string.
 *
 * Restores:
 * `(` from `&#40`,
 * ` ` from `&nbsp;`,
 *
 * Removes spaces around punctuation marks and digits.
 */
export const finalize = (newLine: string): TaraskStep =>
	mutatingStep(({ text }) =>
		text
			.replace(/&#40/g, '(')
			.replace(/&nbsp;/g, ' ')
			.replace(/ (\p{P}|\p{S}|\d+) /gu, '$1')
			.replace(/\n/g, newLine)
			.trim()
	);
