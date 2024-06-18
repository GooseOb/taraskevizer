import { replaceG, ansiColorWrappers, htmlWrappers } from '../lib/index';
import { alphabets, gobj } from '../dict/index';
import type { TaraskStep } from './types';

export const applyGHtml: TaraskStep = (options) => {
	const colorize = htmlWrappers.letterH;
	if (options.cfg.general.abc === alphabets.cyrillic)
		options.text = replaceG(
			options.cfg.html.g ? colorize('$&') : ($0) => colorize(gobj[$0])
		)(options.text);
};

export const applyGNonHtml: TaraskStep = (options) => {
	const colorize = ansiColorWrappers.variable;
	const {
		cfg: {
			general: { abc },
			nonHtml: { h, ansiColors },
		},
	} = options;

	if (abc === alphabets.cyrillic && (h || ansiColors))
		options.text = replaceG(
			ansiColors
				? h
					? ($0) => colorize(gobj[$0])
					: colorize('$&')
				: ($0) => gobj[$0]
		)(options.text);
};
