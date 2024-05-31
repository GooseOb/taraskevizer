import { TaraskConfig } from './config';
import type { TaraskStep } from './steps';

export const tarask = (
	text: string,
	pipeline: TaraskStep<any>[],
	cfg: TaraskConfig = new TaraskConfig()
): string => {
	const options = { cfg, storage: {} };
	return pipeline.reduce((text, step) => step(text, options), text);
};
