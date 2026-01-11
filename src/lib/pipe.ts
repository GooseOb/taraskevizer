import { TaraskConfig } from '../config';
import type { AsyncTaraskStep, TaraskStep } from '../steps';
import type { DeepReadonly } from '../types';

export type Pipeline<Step extends TaraskStep<any> = TaraskStep<any>> = {
	(text: string, cfg?: DeepReadonly<Partial<TaraskConfig>>): string;
	steps: Step[];
};

export type AsyncPipeline = {
	(text: string, cfg?: DeepReadonly<Partial<TaraskConfig>>): Promise<string>;
	steps: AsyncTaraskStep<any>[];
};

/**
 * Create a callable pipeline from steps
 */
export const pipe = <Step extends TaraskStep<any> = TaraskStep<any>>(
	steps: Step[]
) => {
	const fn: Pipeline<Step> = (text, cfg = new TaraskConfig()) => {
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

/**
 * Create a callable async pipeline from steps
 */
export const asyncPipe = <
	Step extends AsyncTaraskStep<any> = AsyncTaraskStep<any>,
>(
	steps: Step[]
) => {
	const fn: AsyncPipeline = async (text, cfg = new TaraskConfig()) => {
		const ctx = {
			text,
			cfg: cfg instanceof TaraskConfig ? cfg : new TaraskConfig(cfg),
			storage: {},
		};
		for (const step of fn.steps) await step(ctx);
		return ctx.text;
	};
	fn.steps = steps;
	return fn;
};
