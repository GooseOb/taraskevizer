import { ansiColorWrappers } from '../lib';
import { VARIATION } from '../constants';
import type { TaraskStep } from './types';

/**
 * @param callback - A function that takes an
 * array of variable parts and returns a string.
 *
 * The step replaces strings like `"(a|b|c)"`
 * with the result of the callback.
 *
 * @see {@link applyVariationsHtml}
 * @see {@link applyVariationsNonHtml}
 */
export const applyVariableParts =
	(callback: (arr: string[]) => string): TaraskStep =>
	(options) => {
		options.text = options.text.replace(/\([^)]*?\)/g, ($0) =>
			callback($0.slice(1, -1).split('|'))
		);
	};

/**
 * Uses {@link applyVariableParts}
 *
 * @example
 * applyVariationsHtml({text: "a(b|c)d"});
 * // {text: "a<tarL data-l='c'>b</tarL>d"}
 */
export const applyVariationsHtml: TaraskStep = applyVariableParts((parts) => {
	const main = parts.shift();
	return `<tarL data-l='${parts}'>${main}</tarL>`;
});

/**
 * Uses {@link applyVariableParts}
 */
export const applyVariationsNonHtml: TaraskStep = (options) => {
	const partIndex = options.cfg.nonHtml.variations;
	const colorize = ansiColorWrappers.variable;

	if (partIndex !== VARIATION.ALL)
		applyVariableParts(
			options.cfg.nonHtml.ansiColors
				? (parts) => colorize(parts[partIndex])
				: (parts) => parts[partIndex]
		)(options);
};
