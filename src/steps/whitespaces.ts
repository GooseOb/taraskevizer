import type { TaraskStep } from './types';

type WhiteSpaceStorage = { spaces: string[] };

export const whitespacesToSpaces: TaraskStep<WhiteSpaceStorage> = (
	text,
	{ storage }
) => {
	storage.spaces = [];
	return text.replace(/\s+/g, (match) => {
		storage.spaces.push(match);
		return ' ';
	});
};

export const restoreWhitespaces: TaraskStep<WhiteSpaceStorage> = (
	text,
	{ storage }
) => {
	storage.spaces.reverse();
	return text.replace(/ /g, () => storage.spaces.pop()!);
};
