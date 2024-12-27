import type { DeepPartialReadonly } from '../types';
import type { TaraskStep } from '../steps';
import { TaraskConfig } from '../config';

export type Pipeline = {
	(text: string, cfg?: DeepPartialReadonly<TaraskConfig>): string;
	steps: TaraskStep<any>[];
};

/**
 * Create a callable pipeline from steps.
 */
export const pipe = (steps: TaraskStep<any>[]) => {
	const fn: Pipeline = (text, cfg = new TaraskConfig()) => {
		const ctx = {
			text,
			cfg: cfg instanceof TaraskConfig ? cfg : new TaraskConfig(cfg),
			storage: {},
		};
		for (const step of fn.steps) step(ctx);
		return ctx.text;
	};
	fn.steps = steps;
	return fn;
};
