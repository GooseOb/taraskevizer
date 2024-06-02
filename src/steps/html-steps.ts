import { alphabets } from '../dict';
import { htmlWrappers, replaceG } from '../lib';
import { applyVariableParts } from './resolve-syntax';
import { gobj } from '../dict';
import type { TaraskStep } from './types';

export const applyHtmlVariations: TaraskStep = applyVariableParts((parts) => {
	const main = parts.shift();
	return `<tarL data-l='${parts}'>${main}</tarL>`;
});

export const applyHtmlG: TaraskStep = (
	text,
	{
		cfg: {
			general: { abc },
			html: { g },
		},
	}
) => {
	const colorize = htmlWrappers.letterH;
	return abc === alphabets.cyrillic
		? replaceG(g ? colorize('$&') : ($0) => colorize(gobj[$0]))(text)
		: text;
};
