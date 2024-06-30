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
	type TaraskStep,
	iotacizeJi,
	untrim,
	applyG,
	applyVariations,
} from './steps/index';

/**
 * Storage for the pipeline {@link abc}.
 */
type AbcOnlyStorage = {
	doEscapeCapitalized: boolean;
};

/**
 * Pipeline for changing only the alphabet.
 *
 * The property `cfg.general.doEscapeCapitalized` is set to `false` during the pipeline execution.
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
 * Pipeline for taraskevizing.
 */
export const tar = [
	trim,
	resolveSpecialSyntax,
	prepare,
	whitespacesToSpaces,
	storeSplittedAbcConvertedOrig,
	toLowerCase,
	taraskevize,
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
 * Pipeline for phonetizing.
 * @alpha
 */
export const phonetic = tar.flatMap((item) =>
	item === taraskevize ? [phonetize, iotacizeJi] : item
);
