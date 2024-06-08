import type { TaraskStep } from './types';

/**
 * Created in {@link whitespacesToSpaces}.
 *
 * Emptied in {@link restoreWhitespaces}.
 */
export type WhiteSpaceStorage = { spaces: string[] };

/**
 * Captures all whitespaces and replaces them with a single space.
 *
 * Creates storage: {@link WhiteSpaceStorage}.
 */
export const whitespacesToSpaces: TaraskStep<WhiteSpaceStorage> = (options) => {
	const { storage } = options;
	storage.spaces = [];
	options.text = options.text.replace(/\s+/g, (match) => {
		storage.spaces.push(match);
		return ' ';
	});
};

/**
 * Brings original whitespaces back to the text.
 *
 * Empties {@link WhiteSpaceStorage.spaces}.
 */
export const restoreWhitespaces: TaraskStep<WhiteSpaceStorage> = (options) => {
	const { spaces } = options.storage;
	spaces.reverse();
	options.text = options.text.replace(/ /g, () => spaces.pop()!);
};
