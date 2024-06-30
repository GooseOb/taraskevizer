import { VARIATION } from './constants';
import { ValueOf } from './types';

type VariationWrappers = {
	[p in ValueOf<typeof VARIATION>]: (content: string) => string;
};

export type WrapperDict = {
	fix?: (content: string) => string;
	variable?: VariationWrappers;
	letterH?: (content: string) => string;
};

export const defaultVariationWrappers = {
	[VARIATION.ALL]: (content) => content,
	[VARIATION.FIRST]: (content) => /^[^|]*?\|([^|)]*)/.exec(content)![1],
	[VARIATION.NO]: (content) => /^\(([^|]*)/.exec(content)![1],
} satisfies VariationWrappers;

export const htmlWrappers = {
	fix: (content) => `<tarF>${content}</tarF>`,
	variable: {
		[VARIATION.ALL]: (content: string) => {
			const [main, ...parts] = content.slice(1, -1).split('|');
			return `<tarL data-l='${parts}'>${main}</tarL>`;
		},
		[VARIATION.FIRST]: (content) => {
			const [first, main, ...parts] = content.slice(1, -1).split('|');
			parts.push(first);
			return `<tarL data-l='${parts}'>${main}</tarL>`;
		},
		[VARIATION.NO]: defaultVariationWrappers[VARIATION.NO],
	},
	letterH: (content) => `<tarH>${content}</tarH>`,
} satisfies WrapperDict;

export const ansiColorWrappers = {
	fix: (content) => `\x1b[32m${content}\x1b[0m`,
	variable: (() => {
		const wrappers = {} as VariationWrappers;

		for (const key in defaultVariationWrappers) {
			type Key = keyof VariationWrappers;
			const getPart = defaultVariationWrappers[key as any as Key];
			wrappers[key as any as Key] = (content) =>
				`\x1b[35m${getPart(content)}\x1b[0m`;
		}

		return wrappers;
	})(),
	letterH: (content) => `\x1b[35m${content}\x1b[0m`,
} satisfies WrapperDict;
