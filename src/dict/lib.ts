import type { CallableDict, Dict } from './types';

export const copyDict = <T extends [any, any][]>(dict: T): T =>
	dict.map(({ 0: pattern, 1: result }) => [pattern, result]) as T;

/**
 * @returns function with property `value` that references the dictionary
 * passed as an argument.
 *
 * It is possible to change the dictionary after initialization by modifying
 * the `value` property.
 *
 * You can use {@link copyDict} before passing the dictionary to this function
 */
export const callableDict = (value: Dict): CallableDict => {
	const fn: CallableDict = (text) =>
		fn.value.reduce((acc, item) => acc.replace(item[0], item[1]), text);
	fn.value = value;
	return fn;
};

export const toOneLine = (str: string): string => str.replace(/\n/g, '|');
