import { TaraskStep } from '../steps/types';

export const mutatingStep =
	<T extends object>(
		callback: (...args: [string, ...Parameters<TaraskStep<T>>]) => string
	): TaraskStep<T> =>
	(options) => {
		options.text = callback(options.text, options);
	};
