import { restoreCase, mutatingStep } from '@/lib';
import type { TaraskStep } from './types';
import type { Alphabet } from '@/dict/alphabets/types';

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
export const applyNoFix: TaraskStep<SpecialSyntaxStorage> = (ctx) => {
	const { noFixArr } = ctx.storage;
	if (noFixArr.length) {
		noFixArr.reverse();
		ctx.text = ctx.text.replaceAll(
			ctx.cfg.noFixPlaceholder,
			() => noFixArr.pop()!
		);
	}
};

/**
 * Captures noFix parts and stores them in {@link SpecialSyntaxStorage.noFixArr}.
 * Places a {@link TaraskConfig.noFixPlaceholder} in their place.
 *
 * Use {@link applyNoFix} to bring the parts back to the text.
 *
 * Creates storage: {@link SpecialSyntaxStorage}.
 */
export const resolveSpecialSyntax = mutatingStep<SpecialSyntaxStorage>(
	({
		text,
		storage,
		cfg: { doEscapeCapitalized, abc, leftAngleBracket, noFixPlaceholder },
	}) => {
		const noFixArr = (storage.noFixArr = [] as string[]);
		const convertAlphavet = (abcOnlyText: string, abc: Alphabet) =>
			restoreCase(
				abc.lower(abcOnlyText.toLowerCase()).split(' '),
				abcOnlyText.split(' ')
			).join(' ');

		const escapeCapsIfNeeded = (text: string) =>
			doEscapeCapitalized
				? text.replace(
						/(?!<=\p{Lu} )\p{Lu}{2}[\p{Lu} ]*(?!= \p{Lu})/gu,
						($0) => {
							noFixArr.push(convertAlphavet($0, abc));
							return noFixPlaceholder;
						}
					)
				: text;
		const parts = text.split(/(?=[<>])/g);
		if (parts.length === 1) return escapeCapsIfNeeded(text);

		let result = text.startsWith('<') ? '' : escapeCapsIfNeeded(parts.shift()!);
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
					let toAddToResult: string;
					switch (char) {
						case '.':
							toAddToResult = noFixPlaceholder;
							noFixArr.push(currentPart);
							break;
						case ',':
							toAddToResult = leftAngleBracket + currentPart + '>';
							break;
						default:
							toAddToResult = leftAngleBracket + noFixPlaceholder;
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
