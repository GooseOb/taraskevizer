import {
	wordlist,
	softeners,
	arabLetters,
	latinLetters,
	latinLettersUpperCase,
	gobj,
	latinLettersJi,
	latinLettersUpperCaseJi,
} from './dict';
import type {
	AlphabetDependentDict,
	ExtendedDict,
	NonHtmlOptions,
	HtmlOptions,
	TaraskOptions,
	DeepPartialReadonly,
	Alphabet,
	OptionJ,
	Variation,
} from './types';
import { diffChars } from 'diff';
import * as debug from './tools.debug';

const isUpperCase = (str: string): boolean => str === str.toUpperCase();

const getLastLetter = (word: string, i: number) => {
	for (let i = word.length - 1; i >= 0; i--)
		if (/\p{L}/u.test(word[i])) return word[i];
	throw new Error(`the last letter of the word ${word} not found. index: ${i}`);
};

const NOFIX_CHAR = ' \ue0fe ';
const NOFIX_REGEX = new RegExp(NOFIX_CHAR, 'g');
const OPTIONAL_WORDS_REGEX = /\([^)]*?\)/g;
const G_REGEX = /[Ґґ]/g;
type G_REGEX_MATCH = 'Ґ' | 'ґ';

export const ALPHABET = {
	CYRILLIC: 0,
	LATIN: 1,
	ARABIC: 2,
	LATIN_JI: 3,
} as const satisfies Record<string, Alphabet>;
export const REPLACE_J = {
	NEVER: 0,
	RANDOM: 1,
	ALWAYS: 2,
} as const satisfies Record<string, OptionJ>;
export const VARIATION = {
	NO: 0,
	FIRST: 1,
	ALL: 2,
} as const satisfies Record<string, Variation>;

const letters: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLetters,
	[ALPHABET.ARABIC]: arabLetters,
	[ALPHABET.LATIN_JI]: latinLettersJi,
};
const lettersUpperCase: AlphabetDependentDict = {
	[ALPHABET.LATIN]: latinLettersUpperCase,
	[ALPHABET.LATIN_JI]: latinLettersUpperCaseJi,
};

const wrappers = {
	html: {
		fix: (content) => `<tarF>${content}</tarF>`,
		letterH: (content) => `<tarH>${content}</tarH>`,
	},
	ansiColors: {
		fix: (content) => `\x1b[32m${content}\x1b[0m`,
		variable: (content) => `\x1b[35m${content}\x1b[0m`,
	},
} satisfies Record<
	'html' | 'ansiColors',
	{ [p in 'fix' | 'letterH' | 'variable']?: (content: string) => string }
>;

const iaReplacer = ($0: string, $1: string, $2: string) =>
	$2.match(/[аеёіоуыэюя]/g)?.length === 1 ? $1 + 'я' + $2 : $0;

const afterTarask: ExtendedDict = [
	[/( б)е(зь? \S+)/g, iaReplacer],
	[/( н)е( \S+)/g, iaReplacer],
	[
		/( (?:б[ея]|пра|цера)?з) і(\S*)/g,
		($0, $1, $2) => (/([ая]ў|ну)$/.test($2) ? $1 + 'ь і' + $2 : $0),
	],
];

const applyNoFix = (arr: string[], text: string) =>
	arr.length ? text.replace(NOFIX_REGEX, () => arr.shift()!) : text;

const afterJoin = (text: string) =>
	text.replace(/&nbsp;/g, ' ').replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1');
const join = (textArr: readonly string[]): string =>
	afterJoin(textArr.join(' '));
const finalize = (text: string, newLine: string) =>
	text.replace(/ \t /g, '\t').replace(/ \n /g, newLine).trim();

const replaceG = (
	text: string,
	replacer: string | ((g: G_REGEX_MATCH) => string)
) =>
	text.replace(
		G_REGEX,
		// @ts-ignore
		replacer
	);

const restoreCase = (text: string[], orig: string[]): string[] => {
	for (let i = 0; i < text.length; i++) {
		const word = text[i];
		const oWord = orig[i];
		if (word === oWord) continue;
		if (word === oWord.toLowerCase()) {
			text[i] = oWord;
			continue;
		}
		if (!oWord[0] || !isUpperCase(oWord[0])) continue;
		if (word === 'зь') {
			text[i] = isUpperCase(orig[i + 1]) ? 'ЗЬ' : 'Зь';
		} else if (isUpperCase(getLastLetter(oWord, i))) {
			text[i] = word.toUpperCase();
		} else {
			text[i] =
				word[0] === '('
					? word.replace(/[^)]*?(?=\))/, ($0) =>
							$0.replace(/[(|]./g, ($0) => $0.toUpperCase())
						)
					: word[0].toUpperCase() + word.slice(1);
		}
	}

	return text;
};

const highlightChanges = (
	text: string,
	orig: string,
	highlight: (content: string) => string
): string =>
	diffChars(orig, text)
		.reduce(
			(acc, diff) =>
				acc +
				(diff.removed
					? '\ufffd'
					: diff.added
						? highlight(diff.value)
						: diff.value),
			''
		)
		.replace(/([^<>\x1b])\ufffd([^<>\x1b])/g, ($0, $1, $2) =>
			highlight($1 + $2)
		)
		.replace(/\ufffd/g, '');

const replaceWithDict = (text: string, dict: ExtendedDict = []) => {
	for (const [pattern, result] of dict)
		text = text.replace(
			pattern,
			//@ts-ignore
			result
		);

	return text;
};

const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

const replaceIbyJ = (text: string, always = false) =>
	text.replace(
		/(?<=[аеёіоуыэюя] )і (ў?)/g,
		always
			? ($0, $1) => toJ($1)
			: ($0, $1) => (Math.random() >= 0.5 ? toJ($1) : $0)
	);

export const __tarask__ = {
	wordlist,
	softeners,
	replaceWithDict,
	afterTarask,
} as const;

const convertAlphabet = (text: string, abc: Alphabet) =>
	replaceWithDict(replaceWithDict(text, letters[abc]), lettersUpperCase[abc]);

export class Taraskevizer {
	public abc: Alphabet = ALPHABET.CYRILLIC;
	public j: OptionJ = REPLACE_J.NEVER;
	public doEscapeCapitalized = true;
	public html = {
		g: false,
	};
	public nonHtml = {
		h: false,
		ansiColors: false,
		variations: VARIATION.ALL as Variation,
	};

	constructor(
		options?: DeepPartialReadonly<{
			general: TaraskOptions;
			html: HtmlOptions;
			nonHtml: NonHtmlOptions;
			OVERRIDE_taraskevize(this: Taraskevizer, text: string): string;
		}>
	) {
		if (!options) return;
		const general = options.general;
		if (general) {
			for (const prop of [
				'abc',
				'j',
				'doEscapeCapitalized',
			] satisfies (keyof TaraskOptions)[])
				if (prop in general) this[prop] = general[prop] as never;
		}
		if (options.OVERRIDE_taraskevize)
			this.taraskevize = options.OVERRIDE_taraskevize;
		Object.assign(this.html, options.html);
		Object.assign(this.nonHtml, options.nonHtml);
	}

	public convert(text: string) {
		const wrapInColorOf = wrappers.ansiColors;
		const noFixArr: string[] = [];
		text = this.prepare(text, noFixArr, '<');
		const origInTargetAlphabet = convertAlphabet(text, this.abc);
		text = this.process(text, origInTargetAlphabet);
		if (this.nonHtml.ansiColors)
			text = highlightChanges(
				text,
				afterJoin(origInTargetAlphabet),
				wrapInColorOf.fix
			);
		if (
			this.abc === ALPHABET.CYRILLIC &&
			(this.nonHtml.h || this.nonHtml.ansiColors)
		)
			text = replaceG(
				text,
				this.nonHtml.ansiColors
					? this.nonHtml.h
						? ($0) => wrapInColorOf.variable(gobj[$0])
						: wrapInColorOf.variable('$&')
					: ($0) => gobj[$0]
			);

		if (
			'variations' in this.nonHtml &&
			this.nonHtml.variations !== VARIATION.ALL
		) {
			const wordIndex = this.nonHtml.variations ?? 0;
			const replacer = ($0: string) => $0.slice(1, -1).split('|')[wordIndex];
			text = text.replace(
				OPTIONAL_WORDS_REGEX,
				this.nonHtml.ansiColors
					? ($0) => wrapInColorOf.variable(replacer($0))
					: replacer
			);
		}

		return finalize(applyNoFix(noFixArr, text).replace(/&#40/g, '('), '\n');
	}

	public convertToHtml(text: string) {
		const wrapInTag = wrappers.html;
		const noFixArr: string[] = [];
		text = this.prepare(text, noFixArr, '&lt;');
		const origInTargetAlphabet = convertAlphabet(text, this.abc);
		text = highlightChanges(
			this.process(text, origInTargetAlphabet),
			afterJoin(origInTargetAlphabet),
			wrapInTag.fix
		);
		if (this.abc === ALPHABET.CYRILLIC)
			text = replaceG(
				text,
				this.html.g
					? wrapInTag.letterH('$&')
					: ($0) => wrapInTag.letterH(gobj[$0])
			);

		return finalize(
			applyNoFix(noFixArr, text).replace(OPTIONAL_WORDS_REGEX, ($0) => {
				const options = $0.slice(1, -1).split('|');
				const main = options.shift();
				return `<tarL data-l='${options}'>${main}</tarL>`;
			}),
			'<br>'
		);
	}

	private prepare(
		text: string,
		noFixArr: string[],
		LEFT_ANGLE_BRACKET: string,
		doEscapeCapitalized = this.doEscapeCapitalized
	) {
		text = ` ${text.trim()} `.replace(/\ue0ff/g, '');
		if (doEscapeCapitalized)
			text = text.replace(
				/(?!<=\p{Lu} )(\p{Lu}{2}[\p{Lu} ]*)(?!= \p{Lu})/gu,
				'<*.$1>'
			);
		return text
			.replace(/<(\*?)([,.]?)([^>]*?)>/gs, ($0, $1, $2, $3) => {
				if ($2 === ',') return LEFT_ANGLE_BRACKET + $3 + '>';
				if ($1)
					$3 = restoreCase(
						[replaceWithDict($3.toLowerCase(), letters[this.abc])],
						[$3]
					);
				noFixArr.push($2 === '.' ? $3 : LEFT_ANGLE_BRACKET + $3 + '>');
				return NOFIX_CHAR;
			})
			.replace(/г'(?![еёіюя])/g, 'ґ')
			.replace(/([\n\t])/g, ' $1 ')
			.replace(/ - /g, ' — ')
			.replace(/(\p{P}|\p{S}|\d)/gu, ' $1 ')
			.replace(/ ['`’] (?=\S)/g, 'ʼ')
			.replace(/\(/g, '&#40');
	}

	public convertAlphabetOnly(text: string) {
		const noFixArr: string[] = [];
		return finalize(
			applyNoFix(
				noFixArr,
				convertAlphabet(this.prepare(text, noFixArr, '<', false), this.abc)
			)
				.replace(/&nbsp;/g, ' ')
				.replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1'),
			'\n'
		);
	}

	private process(text: string, textInTargetAlphabet: string): string {
		const { abc, j } = this;

		const splittedOrig = textInTargetAlphabet.split(' ');

		text = this.taraskevize(text.toLowerCase());
		if (j && abc !== ALPHABET.LATIN_JI)
			text = replaceIbyJ(text, j === REPLACE_J.ALWAYS);
		text = replaceWithDict(text, letters[abc]);

		let splitted = text.split(' ');
		if (abc !== ALPHABET.ARABIC) splitted = restoreCase(splitted, splittedOrig);
		return join(splitted);
	}

	protected taraskevize(text: string) {
		text = replaceWithDict(text, wordlist);
		softening: do {
			text = replaceWithDict(text, softeners);
			for (const [pattern, result] of softeners)
				if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
			break;
		} while (true);

		return replaceWithDict(text, afterTarask);
	}
}
