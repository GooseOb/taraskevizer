import { TaraskConfig } from './config';
import type { TaraskStep } from './steps/index';

/**
 * @returns converted text
 *
 * @example
 * tarask("планета", pipelines.tar);
 * // "плянэта"
 */
export const tarask = (
	text: string,
	pipeline: TaraskStep<any>[],
	cfg: Readonly<TaraskConfig> = new TaraskConfig()
): string => {
	const options = { text, cfg, storage: {} };
	for (const step of pipeline) step(options);
	return options.text;
};
