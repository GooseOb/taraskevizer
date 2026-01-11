import type { TaraskStep, AsyncTaraskStep } from '@/steps/types';

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
	<TStorage extends object = object>(
		callback: (...args: Parameters<TaraskStep<TStorage>>) => string
	): TaraskStep<TStorage> =>
	(ctx) => {
		ctx.text = callback(ctx);
	};

export const mutatingAsyncStep =
	<TStorage extends object = object>(
		callback: (
			...args: Parameters<AsyncTaraskStep<TStorage>>
		) => Promise<string>
	): AsyncTaraskStep<TStorage> =>
	(ctx) =>
		callback(ctx).then((result) => {
			ctx.text = result;
		});
