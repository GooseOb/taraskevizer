import { replaceG } from '../lib';
import { alphabets, gobj } from '../dict';
import type { TaraskStep } from './types';

export const applyG: TaraskStep = (ctx) => {
	const { abc, g, wrapperDict } = ctx.cfg;
	const colorize = wrapperDict?.letterH;

	if (abc === alphabets.cyrillic && (colorize || !g))
		ctx.text = replaceG(
			colorize
				? g
					? colorize('$&')
					: ($0) => colorize(gobj[$0])
				: ($0) => gobj[$0]
		)(ctx.text);
};
