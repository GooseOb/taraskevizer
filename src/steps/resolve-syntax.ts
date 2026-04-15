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
	({ text, storage, cfg: { doEscapeCapitalized, abc, noFixPlaceholder } }) => {
		const noFixArr = (storage.noFixArr = [] as string[]);
		const convertAlphavet = (abcOnlyText: string, abc: Alphabet) =>
			restoreCase(
				abc.lower(abcOnlyText.toLowerCase()).split(' '),
				abcOnlyText.split(' ')
			).join(' ');

		text = text.replace(
			doEscapeCapitalized
				? /<(.*?[^\\])>|(?!<=\p{Lu} )\p{Lu}{2}[\p{Lu} ]*(?!= \p{Lu})/gu
				: /<(.*?[^\\])>/g,
			($0, $1) => {
				let result: string = noFixPlaceholder;
				if ($0.startsWith('<')) {
					let inner = $1 as string;
					const isAbc = +inner.startsWith('*');
					const char = isAbc ? inner[1] : inner[0];
					const doRemoveBrackets = +(char === '.');
					const doTarask = +(char === ',');
					inner = inner
						.slice(isAbc + doRemoveBrackets + doTarask)
						.replace(/\\>/g, '>');
					if (doTarask) {
						return `<${inner}>`;
					}
					noFixArr.push(isAbc ? convertAlphavet(inner, abc) : inner);
					result = doRemoveBrackets
						? noFixPlaceholder
						: `<${noFixPlaceholder}>`;
				} else {
					noFixArr.push(convertAlphavet($0, abc));
				}

				return result;
			}
		);

		return text;
	}
);
