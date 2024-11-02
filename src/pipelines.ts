/**
 * Builtin pipelines
 *
 * If you want to create your own pipeline
 * based on a builtin one and do it safely, use
 * `pipeline.steps.map` if you need replacing only or
 * `pipeline.steps.flatMap` if you also need to add or remove steps.
 *
 * Then pass the result to {@link createPipeline}.
 *
 * This way you will depend less on the
 * internal order of the builtin pipeline steps.
 *
 * For cases when you need to replace only {@link steps.taraskevize} in {@link tarask}
 * and have better tree-shaking, you can use {@link _createPipeline}.
 *
 * @module
 */

import { TaraskConfig } from './config';
import {
	type TaraskStep,
	highlightDiffStep,
	applyNoFix,
	convertAlphabet,
	convertAlphabetLowerCase,
	joinSplittedText,
	prepare,
	replaceIbyJ,
	resolveSpecialSyntax,
	restoreCaseStep,
	restoreWhitespaces,
	storeSplittedAbcConvertedOrig,
	storeSplittedText,
	taraskevize,
	phonetize,
	whitespacesToSpaces,
	trim,
	finalize,
	toLowerCase,
	iotacizeJi,
	untrim,
	applyG,
	applyVariations,
} from './steps';

export type Pipeline = {
	(text: string, cfg?: Partial<TaraskConfig>): string;
	steps: TaraskStep<any>[];
};

/**
 * Create a callable pipeline from steps.
 */
export const createPipeline = (steps: TaraskStep<any>[]) => {
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

/**
 * Pipeline for changing only the alphabet.
 *
 * The property {@link TaraskConfig.doEscapeCapitalized} is set to `false` during the pipeline execution.
 *
 * To see the full list of steps, check the source code.
 */
export const alphabetic = createPipeline([
	(ctx) => {
		ctx.cfg = { ...ctx.cfg, doEscapeCapitalized: false };
	},
	trim,
	resolveSpecialSyntax,
	prepare,
	whitespacesToSpaces,
	convertAlphabet,
	restoreWhitespaces,
	applyNoFix,
	finalize,
	untrim,
]);

/**
 * For better tree-shaking instead of `Array.prototype.flatMap`
 *
 * Used by {@link tarask} and {@link phonetic}.
 *
 * @param subPipeline - Steps used instead of [{@link steps.taraskevize}].
 */
export const _createPipeline = (subPipeline: TaraskStep<any>[]) =>
	createPipeline([
		trim,
		resolveSpecialSyntax,
		prepare,
		whitespacesToSpaces,
		storeSplittedAbcConvertedOrig,
		toLowerCase,
		...subPipeline,
		replaceIbyJ,
		convertAlphabetLowerCase,
		storeSplittedText,
		restoreCaseStep,
		highlightDiffStep,
		joinSplittedText,
		restoreWhitespaces,
		applyG,
		applyVariations,
		applyNoFix,
		finalize,
		untrim,
	]);

/**
 * Pipeline for taraskevizing.
 */
export const tarask = _createPipeline([taraskevize]);

/**
 * Pipeline for phonetizing.
 * @alpha
 */
export const phonetic = _createPipeline([phonetize, iotacizeJi]);
