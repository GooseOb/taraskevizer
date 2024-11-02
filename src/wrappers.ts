import type { Variation } from './config';

type TransformString = (content: string) => string;

export type VariationWrappers = {
	[p in Variation]: TransformString;
};

export type Wrappers = {
	fix?: TransformString;
	variable?: VariationWrappers;
	letterH?: TransformString;
};

export const defaultVariation: VariationWrappers = {
	all: (content) => content,
	first: (content) => /^[^|]*?\|([^|)]*)/.exec(content)![1],
	no: (content) => /^\(([^|]*)/.exec(content)![1],
};

export const html: Required<Wrappers> = {
	fix: (content) => `<tarF>${content}</tarF>`,
	variable: {
		all: (content: string) => {
			const [main, ...parts] = content.slice(1, -1).split('|');
			return `<tarL data-l='${parts}'>${main}</tarL>`;
		},
		first: (content) => {
			const [first, main, ...parts] = content.slice(1, -1).split('|');
			parts.push(first);
			return `<tarL data-l='${parts}'>${main}</tarL>`;
		},
		no: defaultVariation.no,
	},
	letterH: (content) => `<tarH>${content}</tarH>`,
};

export const ansiColor: Required<Wrappers> = {
	fix: (content) => `\x1b[32m${content}\x1b[0m`,
	variable: (
		Object.entries(defaultVariation) as [Variation, TransformString][]
	).reduce((acc, [key, getPart]) => {
		acc[key] = (content) => `\x1b[35m${getPart(content)}\x1b[0m`;
		return acc;
	}, {} as VariationWrappers),
	letterH: (content) => `\x1b[35m${content}\x1b[0m`,
};
