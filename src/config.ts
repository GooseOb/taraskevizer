import { alphabets } from './dict';
import type { Alphabet } from './dict';
import type {
	DeepPartialReadonly,
	HtmlOptions,
	NonHtmlOptions,
	OptionJ,
	TaraskOptions,
	Variation,
} from './types';

export class TaraskConfig {
	constructor(
		options?: DeepPartialReadonly<{
			general: TaraskOptions;
			html: HtmlOptions;
			nonHtml: NonHtmlOptions;
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
	public general = {
		abc: alphabets.cyrillic as Alphabet,
		j: REPLACE_J.NEVER as OptionJ,
		doEscapeCapitalized: true,
	};
	public html = {
		g: false,
	};
	public nonHtml = {
		h: false,
		ansiColors: false,
		variations: VARIATION.ALL as Variation,
	};
}

export const REPLACE_J = {
	NEVER: 0,
	RANDOM: 1,
	ALWAYS: 2,
} as const satisfies Record<string, OptionJ>;

export const VARIATION = {
	NO: 0,
	FIRST: 1,
	ALL: 2,
} as const satisfies Record<string, Variation>;
