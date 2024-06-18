import { alphabets } from './dict/index';
import type {
	PartialReadonly,
	HtmlOptions,
	NonHtmlOptions,
	TaraskOptions,
} from './types';
import { REPLACE_J, VARIATION } from './constants';

export class TaraskConfig {
	constructor(
		options?: PartialReadonly<{
			general: PartialReadonly<TaraskOptions>;
			html: PartialReadonly<HtmlOptions>;
			nonHtml: PartialReadonly<NonHtmlOptions>;
		}>
	) {
		if (!options) return;
		for (const [target, source] of [
			[this.general, options.general],
			[this.html, options.html],
			[this.nonHtml, options.nonHtml],
		]) {
			if (source)
				for (const [key, value] of Object.entries(source)) {
					if (value !== undefined) (target as any)[key] = value;
				}
		}
	}

	/** @see {@link TaraskOptions} */
	public general = {
		abc: alphabets.cyrillic,
		j: REPLACE_J.NEVER,
		doEscapeCapitalized: true,
	} as TaraskOptions;

	/** @see {@link HtmlOptions} */
	public html = {
		g: false,
	} as HtmlOptions;

	/** @see {@link NonHtmlOptions} */
	public nonHtml = {
		h: false,
		ansiColors: false,
		variations: VARIATION.ALL,
	} as NonHtmlOptions;
}
