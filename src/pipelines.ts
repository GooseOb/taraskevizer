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
	applyVariationsHtml,
	applyVariationsNonHtml,
	applyGHtml,
	applyGNonHtml,
	highlightDiffStep,
	highlightDiffStepNonHtml,
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
} from './steps/index';
import { htmlWrappers } from './lib/wrappers';

const resolveSpecialSyntaxWithLAB = resolveSpecialSyntax('<');

const finalizeWithNewLine = finalize('\n');

/**
 * Storage for the pipeline {@link abcOnly}.
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
export const abcOnly = [
	(({ storage, cfg: { general } }) => {
		storage.doEscapeCapitalized = general.doEscapeCapitalized;
		general.doEscapeCapitalized = false;
	}) satisfies TaraskStep<AbcOnlyStorage>,
	trim,
	resolveSpecialSyntaxWithLAB,
	prepare,
	whitespacesToSpaces,
	convertAlphabet,
	restoreWhitespaces,
	applyNoFix,
	finalizeWithNewLine,
	untrim,
	(({ storage, cfg: { general } }) => {
		general.doEscapeCapitalized = storage.doEscapeCapitalized;
	}) satisfies TaraskStep<AbcOnlyStorage>,
] satisfies TaraskStep<any>[];

const createPipeline = (
	resolveSpecialSyntax: TaraskStep<any>,
	applyG: TaraskStep<any>,
	applyVariations: TaraskStep<any>,
	finalize: TaraskStep<any>,
	highlightDiffStep: TaraskStep<any>
) =>
	[
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
 * Pipeline for taraskevizing into plain text.
 */
export const plainText = createPipeline(
	resolveSpecialSyntaxWithLAB,
	applyGNonHtml,
	applyVariationsNonHtml,
	finalizeWithNewLine,
	highlightDiffStepNonHtml
);

/**
 * Pipeline for taraskevizing into HTML.
 */
export const html = createPipeline(
	resolveSpecialSyntax('&lt;'),
	applyGHtml,
	applyVariationsHtml,
	finalize('<br>'),
	highlightDiffStep(htmlWrappers.fix)
);

/**
 * Pipeline for phonetizing.
 * @alpha
 */
export const phonetic = plainText.flatMap((item) =>
	item === taraskevize ? [phonetize, iotacizeJi] : item
);
