import { mutatingStep, re } from '@/lib';

/**
 * Reverse the changes made in the {@link prepare} step
 * and replace new lines with the passed string.
 *
 * Restores:
 * `(` from `&#40`,
 * ` ` from `&nbsp;`,
 *
 * Replaces new lines with the `newLine` config value.
 *
 * Removes spaces around punctuation marks and digits.
 */
export const finalize = mutatingStep(
	({ text, cfg: { newLine, leftAngleBracket } }) => {
		text = text
			.replace(/&#40/g, '(')
			.replace(/&nbsp;/g, ' ')
			.replace(/ (\p{P}|\p{S}|\d+) /gu, '$1')
			.replace(re.g(` ${leftAngleBracket} `), leftAngleBracket);

		if (newLine !== '\n') text = text.replace(/\n/g, newLine);

		return text.trim();
	}
);
