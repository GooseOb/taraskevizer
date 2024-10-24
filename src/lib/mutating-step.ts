import type { TaraskStep } from '../steps/types';

/**
 * An abstraction for a step that always changes the text.
 *
 * > Not recommended to use if
 * a step doesn't ALWAYS change the text.
 *
 * @example
 * type TextWrapperStorage = { wrapText: (text: string) => string };
 *
 * const trimStep = mutatingStep<TextWrapperStorage>(
 *   ({ text, storage: { wrapText } }) => wrapText(text.trim())
 * );
 * // is equivalent to
 * const trimStep: TaraskStep<TextWrapperStorage> = (options) => {
 *   options.text = options.storage.wrapText(
 *     options.text.trim()
 *   );
 * };
 */
export const mutatingStep =
	<T extends object = object>(
		callback: (...args: Parameters<TaraskStep<T>>) => string
	): TaraskStep<T> =>
	(options) => {
		options.text = callback(options);
	};
