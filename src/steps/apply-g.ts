import { replaceG } from '@/lib';
import { alphabets, gobj } from '@/dict';
import type { TaraskStep } from './types';

export const applyG: TaraskStep = (ctx) => {
	const { abc, g, wrapperDict } = ctx.cfg;
	const wrap = wrapperDict?.letterH;

	if (abc === alphabets.cyrillic && (wrap || !g))
		ctx.text = replaceG(
			wrap ? (g ? wrap('$&') : ($0) => wrap(gobj[$0])) : ($0) => gobj[$0]
		)(ctx.text);
};
