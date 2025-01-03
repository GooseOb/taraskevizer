import { mutatingStep } from '@/lib';

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
export const whitespacesToSpaces = mutatingStep<WhiteSpaceStorage>(
	({ text, storage }) => {
		storage.spaces = [];
		return text.replace(/\s+/g, ($0) => {
			storage.spaces.push($0);
			return ' ';
		});
	}
);

/**
 * Brings original whitespaces back to the text.
 *
 * Empties {@link WhiteSpaceStorage.spaces}.
 */
export const restoreWhitespaces = mutatingStep<WhiteSpaceStorage>(
	({ text, storage: { spaces } }) => {
		spaces.reverse();
		return text.replace(/ /g, () => spaces.pop()!);
	}
);
