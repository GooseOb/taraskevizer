import { applyVariableParts } from './resolve-syntax';
import { replaceG, ansiColorWrappers } from '../lib';
import { VARIATION } from '../config';
import { ALPHABET } from '../alphabet';
import { gobj } from '../dict';
import type { TaraskStep } from './types';

export const applyNonHtmlVariations: TaraskStep = (text, options) => {
	const partIndex = options.cfg.nonHtml.variations;
	const colorize = ansiColorWrappers.variable;

	return partIndex === VARIATION.ALL
		? text
		: applyVariableParts(
				options.cfg.nonHtml.ansiColors
					? (parts) => colorize(parts[partIndex])
					: (parts) => parts[partIndex]
			)(text, options);
};

export const applyNonHtmlG: TaraskStep = (
	text,
	{
		cfg: {
			general: { abc },
			nonHtml: { h, ansiColors },
		},
	}
) => {
	const colorize = ansiColorWrappers.variable;
	return abc === ALPHABET.CYRILLIC && (h || ansiColors)
		? replaceG(
				ansiColors
					? h
						? ($0) => colorize(gobj[$0])
						: colorize('$&')
					: ($0) => gobj[$0]
			)(text)
		: text;
};
