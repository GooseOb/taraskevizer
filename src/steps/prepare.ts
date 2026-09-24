import { mutatingStep, re } from '@/lib';

/**
 * Prepares the text for correct processing.
 *
 * Some changes should be reverted in the {@link finalize} step.
 */
export const prepare = mutatingStep(({ text, cfg: { leftAngleBracket } }) =>
	text
		.replace(/г'(?![еёіюя])/g, 'ґ')
		.replace(/ - /g, ' — ')
		.replace(re.g(leftAngleBracket), ' $& ')
		.replace(/\p{P}|\p{S}|\d+/gu, ' $& ')
		.replace(/ ['`’] (?=\S)/g, 'ʼ')
		.replace(/\(/g, '&#40')
);
