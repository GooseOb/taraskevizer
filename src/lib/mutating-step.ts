import { TaraskStep } from '../steps/types';

/**
 * An abstraction for a step that always changes the text.
 *
 * > Not recommended to use if
 * a step doesn't ALWAYS change the text.
 */
export const mutatingStep =
	<T extends object = {}>(
		callback: (...args: Parameters<TaraskStep<T>>) => string
	): TaraskStep<T> =>
	(options) => {
		options.text = callback(options);
	};
