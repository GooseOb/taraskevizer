type Wrappers = {
	[p in 'fix' | 'letterH' | 'variable']?: (content: string) => string;
};
export const htmlWrappers = {
	fix: (content) => `<tarF>${content}</tarF>`,
	letterH: (content) => `<tarH>${content}</tarH>`,
} satisfies Wrappers;

export const ansiColorWrappers = {
	fix: (content) => `\x1b[32m${content}\x1b[0m`,
	variable: (content) => `\x1b[35m${content}\x1b[0m`,
} satisfies Wrappers;
