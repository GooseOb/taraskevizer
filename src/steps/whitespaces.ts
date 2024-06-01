import type { TaraskStep } from './types';

type WhiteSpaceStorage = { spaces: string[]; positions: number[] };

export const whitespacesToSpaces: TaraskStep<WhiteSpaceStorage> = (
	text,
	{ storage }
) => {
	storage.spaces = [];
	storage.positions = [];
	let result = '';
	let i = 0;

	while (i < text.length) {
		if (/\s/.test(text[i])) {
			let start = i;
			while (i < text.length && /\s/.test(text[i])) ++i;
			storage.spaces.push(text.slice(start, i));
			storage.positions.push(result.length);
			result += ' ';
		} else {
			result += text[i];
			++i;
		}
	}

	return result;
};

export const restoreWhitespaces: TaraskStep<WhiteSpaceStorage> = (
	text,
	{ storage }
) => {
	let result = '';
	let lastIndex = 0;
	let spaceIndex = 0;

	for (let i = 0; i < text.length; i++) {
		if (text[i] === ' ' && spaceIndex < storage.spaces.length) {
			result += text.slice(lastIndex, i) + storage.spaces[spaceIndex];
			lastIndex = i + 1;
			++spaceIndex;
		}
	}

	result += text.slice(lastIndex);
	return result;
};
