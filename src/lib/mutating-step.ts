import type { TaraskStep } from '@/steps/types';

/**
 * Utility function for a step that always modifies the text
 *
 * > Not recommended to use if
 * a step doesn't ALWAYS modify the text.
 *
 * @example
 * type TextWrapperStorage = { wrapText: (text: string) => string };
 *
 * const trimStep = mutatingStep<TextWrapperStorage>(
 *   ({ text, storage: { wrapText } }) => wrapText(text.trim())
 * );
 * // is equivalent to
 * const trimStep: TaraskStep<TextWrapperStorage> = (ctx) => {
 *   ctx.text = options.storage.wrapText(
 *     ctx.text.trim()
 *   );
 * };
 */
export const mutatingStep =
	<T extends object = object>(
		callback: (...args: Parameters<TaraskStep<T>>) => string
	): TaraskStep<T> =>
	(ctx) => {
		ctx.text = callback(ctx);
	};
