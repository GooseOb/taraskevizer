import { replaceG } from '../lib/index';
import { alphabets, gobj } from '../dict/index';
import type { TaraskStep } from './types';

export const applyG: TaraskStep = (options) => {
	const { abc, g, wrapperDict } = options.cfg;
	const colorize = wrapperDict?.letterH;

	if (abc === alphabets.cyrillic && (colorize || !g))
		options.text = replaceG(
			colorize
				? g
					? colorize('$&')
					: ($0) => colorize(gobj[$0])
				: ($0) => gobj[$0]
		)(options.text);
};
