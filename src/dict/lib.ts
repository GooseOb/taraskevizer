import type { Dict, RawDict, WritableDict } from './types';

export const copyDict = (dict: Dict): Dict =>
	dict.map(([pattern, result]) => [pattern, result]);

/**
 * Collection of MUTATING functions
 * that help to work with dictionaries.
 *
 * Use {@link copyDict} before applying these functions
 * to avoid mutating the original dictionary.
 *
 * @example
 * const rawDict = [
 *   ["pattern", "result"],
 * ];
 *
 * const dict = dictFrom.raw(copyDict(rawDict));
 * // [ [ /pattern/g, "result" ] ]
 */
export const dictFrom = {
	raw: (dict: RawDict, additionalFlags = 'g'): Dict => {
		for (const item of dict)
			item[0] = RegExp(
				item[0],
				item[0] instanceof RegExp
					? item[0].flags + additionalFlags
					: additionalFlags
			);
		return dict as Dict;
	},
	/**
	 * Adds the global flag to all patterns in the dictionary.
	 */
	nonGlobal: (dict: WritableDict): Dict => {
		for (const item of dict) item[0] = RegExp(item[0], 'g' + item[0].flags);
		return dict;
	},
} satisfies Record<string, (dictLike: WritableDict) => Dict>;

export const toOneLine = (str: string): string => str.replace(/\n/g, '|');
