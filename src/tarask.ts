import { TaraskConfig } from './config';
import type { Pipeline } from './pipelines';

/**
 * @returns converted text
 *
 * @example
 * tarask("планета", pipelines.tar);
 * // "плянэта"
 */
export const tarask = (
	text: string,
	pipeline: Pipeline,
	cfg: Readonly<TaraskConfig> = new TaraskConfig()
): string => {
	const ctx = { text, cfg, storage: {} };
	for (const step of pipeline) step(ctx);
	return ctx.text;
};
