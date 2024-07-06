/**
 * A pipeline in Taraskevizer is basically
 * an array of steps of type {@link steps.TaraskStep}.
 *
 * You can use pre-made pipelines from this module or create your own.
 *
 * If you want to create a pipeline based on a pre-made one
 * and do it more safely, it's recommended to use
 * `.map` if you need replacing only or
 * `.flatMap` if you also need to add or remove steps.
 *
 * This way you will depend less on the
 * internal structure of the pre-made pipeline.
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

/**
 * Storage for the pipeline {@link abc}.
 */
type AbcOnlyStorage = {
	doEscapeCapitalized: boolean;
};

/**
 * Pipeline for changing only the alphabet.
 *
 * The property {@link TaraskConfig.doEscapeCapitalized} is set to `false` during the pipeline execution.
 *
 * To see the full list of steps, check the source code.
 */
export const abc = [
	(({ storage, cfg }) => {
		storage.doEscapeCapitalized = cfg.doEscapeCapitalized;
		cfg.doEscapeCapitalized = false;
	}) satisfies TaraskStep<AbcOnlyStorage>,
	trim,
	resolveSpecialSyntax,
	prepare,
	whitespacesToSpaces,
	convertAlphabet,
	restoreWhitespaces,
	applyNoFix,
	finalize,
	untrim,
	(({ storage, cfg }) => {
		cfg.doEscapeCapitalized = storage.doEscapeCapitalized;
	}) satisfies TaraskStep<AbcOnlyStorage>,
] satisfies TaraskStep<any>[];

/**
 * For better tree-shaking instead of `Array.prototype.flatMap`
 *
 * Used by {@link tar} and {@link phonetic}.
 *
 * @param steps - Steps used instead of [{@link steps.taraskevize}].
 */
export const _createPipeline = (steps: TaraskStep<any>[]) =>
	[
		trim,
		resolveSpecialSyntax,
		prepare,
		whitespacesToSpaces,
		storeSplittedAbcConvertedOrig,
		toLowerCase,
		...steps,
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
	] satisfies TaraskStep<any>[];

/**
 * Pipeline for taraskevizing.
 */
export const tar = _createPipeline([taraskevize]);

/**
 * Pipeline for phonetizing.
 * @alpha
 */
export const phonetic = _createPipeline([phonetize, iotacizeJi]);
