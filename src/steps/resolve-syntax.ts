import { restoreCase, replaceWithDict, mutatingStep } from '../lib';
import type { TaraskStep } from './types';
import type { Alphabet } from '../dict/alphabets';

const NOFIX_CHAR = ' \ue0fe ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');

/**
 * Created in {@link resolveSpecialSyntax}.
 *
 * Emptied in {@link applyNoFix}.
 */
export type SpecialSyntaxStorage = {
	noFixArr: string[];
};

/**
 * Brings parts from {@link SpecialSyntaxStorage} back to the text.
 *
 * Empties {@link SpecialSyntaxStorage.noFixArr}.
 */
export const applyNoFix: TaraskStep<SpecialSyntaxStorage> = (options) => {
	const { noFixArr } = options.storage;
	if (noFixArr.length) {
		noFixArr.reverse();
		options.text = options.text.replace(NOFIX_REGEX, () => noFixArr.pop()!);
	}
};

/**
 * Captures noFix parts and stores them in {@link SpecialSyntaxStorage.noFixArr}.
 * Places a special character `" \ue0fe "` in their place.
 *
 * Use {@link applyNoFix} to bring the parts back to the text.
 *
 * Creates storage: {@link SpecialSyntaxStorage}.
 */
export const resolveSpecialSyntax = (leftAngleBracket: string) =>
	mutatingStep<SpecialSyntaxStorage>(
		({
			text,
			storage,
			cfg: {
				general: { doEscapeCapitalized, abc },
			},
		}) => {
			const noFixArr = (storage.noFixArr = [] as string[]);
			const convertAlphavet = (abcOnlyText: string, abc: Alphabet) =>
				restoreCase(
					replaceWithDict(abcOnlyText.toLowerCase(), abc.lower).split(' '),
					abcOnlyText.split(' ')
				).join(' ');

			const escapeCapsIfNeeded = (text: string) =>
				doEscapeCapitalized
					? text.replace(
							/(?!<=\p{Lu} )\p{Lu}{2}[\p{Lu} ]*(?!= \p{Lu})/gu,
							($0) => {
								noFixArr.push(convertAlphavet($0, abc));
								return NOFIX_CHAR;
							}
					  )
					: text;
			const parts = text.split(/(?=[<>])/g);
			if (parts.length === 1) return escapeCapsIfNeeded(text);

			let result = text.startsWith('<')
				? ''
				: escapeCapsIfNeeded(parts.shift()!);
			let depth = 0;
			let currentPart = '';
			for (const part of parts) {
				if (part.startsWith('<')) {
					++depth;
					currentPart += part;
				} else if (depth) {
					--depth;
					if (depth) {
						currentPart += part;
					} else {
						let char = '';
						const isAbc = currentPart[1] === '*';
						if (isAbc) {
							char = currentPart[2];
							currentPart = convertAlphavet(
								currentPart.slice(char === ',' || char === '.' ? 3 : 2),
								abc
							);
						} else {
							char = currentPart[1];
							currentPart = currentPart.slice(2);
						}
						let toAddToResult;
						switch (char) {
							case '.':
								toAddToResult = NOFIX_CHAR;
								noFixArr.push(currentPart);
								break;
							case ',':
								toAddToResult = leftAngleBracket + currentPart + '>';
								break;
							default:
								toAddToResult = leftAngleBracket + NOFIX_CHAR;
								noFixArr.push((isAbc ? '' : char) + currentPart + '>');
						}
						result += toAddToResult + escapeCapsIfNeeded(part.slice(1));
						currentPart = '';
					}
				} else {
					result += escapeCapsIfNeeded(part);
				}
			}
			return result + escapeCapsIfNeeded(currentPart);
		}
	);
