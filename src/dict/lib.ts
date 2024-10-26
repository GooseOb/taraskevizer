import type { CallableDict, Dict } from './types';

export const copyDict = <T extends [any, any][]>(dict: T): T =>
	dict.map(({ 0: pattern, 1: result }) => [pattern, result]) as T;

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
 * const dict = dict.raw(copyDict(rawDict));
 * // [ [ /pattern/g, "result" ] ]
 */
export const callableDict = (dict: Dict): CallableDict => {
	const fn: CallableDict = (text) =>
		fn.value.reduce((acc, item) => acc.replace(item[0], item[1]), text);
	fn.value = dict;
	return fn;
};

export const toOneLine = (str: string): string => str.replace(/\n/g, '|');

export const regexG = (pattern: string) => new RegExp(pattern, 'g');
export const regexGI = (pattern: string) => new RegExp(pattern, 'gi');
