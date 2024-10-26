/**
 * You can use builtin pipelines from this module or create your own.
 *
 * If you want to create a pipeline based on a builtin one
 * and do it more safely, it's recommended to use
 * `.map` if you need replacing only or
 * `.flatMap` if you also need to add or remove steps.
 *
 * This way you will depend less on the
 * internal structure of the builtin pipeline.
 *
 * For cases when you need to replace only {@link steps.taraskevize} in {@link tar}
 * and have better tree-shaking, you can use {@link _createPipeline}.
 *
 * @module
 */

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

export type Pipeline = TaraskStep<any>[];

/**
 * Pipeline for changing only the alphabet.
 *
 * The property {@link TaraskConfig.doEscapeCapitalized} is set to `false` during the pipeline execution.
 *
 * To see the full list of steps, check the source code.
 */
export const abc = [
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
] satisfies Pipeline;

/**
 * For better tree-shaking instead of `Array.prototype.flatMap`
 *
 * Used by {@link tar} and {@link phonetic}.
 *
 * @param subPipeline - Steps used instead of [{@link steps.taraskevize}].
 */
export const _createPipeline = (subPipeline: Pipeline) =>
	[
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
	] satisfies Pipeline;

/**
 * Pipeline for taraskevizing.
 */
export const tar = _createPipeline([taraskevize]);

/**
 * Pipeline for phonetizing.
 * @alpha
 */
export const phonetic = _createPipeline([phonetize, iotacizeJi]);
