import {
	applyHtmlG,
	applyHtmlVariations,
	applyNoFix,
	applyNonHtmlG,
	applyNonHtmlVariations,
	convertAlphabet,
	convertAlphabetLowerCase,
	highlightDiffNonHtmlStep,
	highlightDiffStep,
	joinSplittedText,
	prepare,
	replaceIbyJ,
	resolveSpecialSyntax,
	restoreCaseStep,
	restoreWhitespaces,
	storeSplittedAbcConvertedOrig,
	storeSplittedText,
	taraskevize,
	whitespacesToSpaces,
	trim,
	type TaraskStep,
} from './steps';
import { htmlWrappers } from './lib/wrappers';

const restoreParentheses: TaraskStep = (text) => text.replace(/&#40/g, '(');
const afterJoin: TaraskStep = (text) =>
	text.replace(/&nbsp;/g, ' ').replace(/ (\p{P}|\p{S}|\d+|&#40) /gu, '$1');
const finalize =
	(newLine: string): TaraskStep =>
	(text) =>
		text.replace(/\n/g, newLine).trim();

const resolveSpecialSyntaxWithLAB = resolveSpecialSyntax('<');
const toLowerCase: TaraskStep = (text) => text.toLowerCase();
const finalizeWithNewLine = finalize('\n');

export const abcOnlyPipeline = [
	((_, { storage, cfg: { general } }) => {
		storage.doEscapeCapitalized = general.doEscapeCapitalized;
		general.doEscapeCapitalized = false;
		return _;
	}) satisfies TaraskStep<{ doEscapeCapitalized: boolean }>,
	trim,
	resolveSpecialSyntaxWithLAB,
	prepare,
	whitespacesToSpaces,
	convertAlphabet,
	restoreWhitespaces,
	applyNoFix,
	restoreParentheses,
	afterJoin,
	finalizeWithNewLine,
	((_, { storage, cfg: { general } }) => {
		general.doEscapeCapitalized = storage.doEscapeCapitalized;
		return _;
	}) satisfies TaraskStep<{ doEscapeCapitalized: boolean }>,
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
		restoreParentheses,
		afterJoin,
		applyG,
		applyVariations,
		applyNoFix,
		finalize,
	] satisfies TaraskStep<any>[];

export const plainTextPipeline = createPipeline(
	resolveSpecialSyntaxWithLAB,
	applyNonHtmlG,
	applyNonHtmlVariations,
	finalizeWithNewLine,
	highlightDiffNonHtmlStep
);

export const htmlPipeline = createPipeline(
	resolveSpecialSyntax('&lt;'),
	applyHtmlG,
	applyHtmlVariations,
	finalize('<br>'),
	highlightDiffStep(htmlWrappers.fix)
);
