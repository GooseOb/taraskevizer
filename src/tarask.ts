import { wordlist, softeners, gobj, noSoften } from './dict';
import { replaceG } from './replace-g';
import { highlightDiff, wrappers } from './highlight-diff';
import { replaceIbyJ } from './i-to-j';
import { restoreCase } from './restore-case';
import { afterTarask } from './after-tarask';
import { ALPHABET, REPLACE_J, VARIATION } from './config-constants';
import { letters } from './letters';
import { replaceWithDict, convertAlphabet } from './tools';
import { resolveSpecialSyntax } from './resolve-syntax';
import type {
	NonHtmlOptions,
	HtmlOptions,
	TaraskOptions,
	DeepPartialReadonly,
	Alphabet,
	OptionJ,
	Variation,
} from './types';
import * as debug from './tools.debug';
import { applyNoFix } from './no-fix';

const OPTIONAL_WORDS_REGEX = /\([^)]*?\)/g;

const afterJoin = (text: string) =>
	text.replace(/&nbsp;/g, ' ').replace(/ (\p{P}|\p{S}|\d|&#40) /gu, '$1');
const join = (textArr: readonly string[]): string =>
	afterJoin(textArr.join(' '));
const finalize = (text: string, newLine: string) =>
	text.replace(/ \t /g, '\t').replace(/ \n /g, newLine).trim();

const restoreBraces = (text: string) => text.replace(/&#40/g, '(');

export class Taraskevizer {
	public general = {
		abc: ALPHABET.CYRILLIC as Alphabet,
		j: REPLACE_J.NEVER as OptionJ,
		doEscapeCapitalized: true,
	};
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
			taraskevize(this: Taraskevizer, text: string): string;
		}>
	) {
		if (!options) return;
		Object.assign(this.general, options.general);
		Object.assign(this.html, options.html);
		Object.assign(this.nonHtml, options.nonHtml);
		if (options.taraskevize) this.taraskevize = options.taraskevize;
	}

	public convert(text: string) {
		const wrapInColorOf = wrappers.ansiColors;
		const isCyrillic = this.general.abc === ALPHABET.CYRILLIC;
		const noFixArr: string[] = [];
		const { splitted, splittedOrig } = this.process(
			this.prepare(text, noFixArr, '<')
		);
		if (this.nonHtml.ansiColors)
			highlightDiff(splitted, splittedOrig, isCyrillic, wrapInColorOf.fix);
		text = join(splitted);
		if (isCyrillic && (this.nonHtml.h || this.nonHtml.ansiColors))
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

		return finalize(restoreBraces(applyNoFix(noFixArr, text)), '\n');
	}

	public convertToHtml(text: string) {
		const wrapInTag = wrappers.html;
		const isCyrillic = this.general.abc === ALPHABET.CYRILLIC;
		const noFixArr: string[] = [];
		const { splitted, splittedOrig } = this.process(
			this.prepare(text, noFixArr, '&lt;')
		);
		highlightDiff(splitted, splittedOrig, isCyrillic, wrapInTag.fix);
		text = join(splitted);
		if (isCyrillic)
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
		doEscapeCapitalized = this.general.doEscapeCapitalized
	) {
		text = ` ${text.trim()} `.replace(/\ue0ff/g, '');
		if (doEscapeCapitalized)
			text = text.replace(
				/(?!<=\p{Lu} )(\p{Lu}{2}[\p{Lu} ]*)(?!= \p{Lu})/gu,
				'<*.$1>'
			);
		return resolveSpecialSyntax(
			text,
			noFixArr,
			LEFT_ANGLE_BRACKET,
			(abcOnlySequence) =>
				restoreCase(
					replaceWithDict(
						abcOnlySequence.toLowerCase(),
						letters[this.general.abc]
					).split(' '),
					abcOnlySequence.split(' ')
				).join(' ')
		)
			.replace(/г'(?![еёіюя])/g, 'ґ')
			.replace(/ - /g, ' — ')
			.replace(/([\n\t]|\p{P}|\p{S}|\d)/gu, ' $1 ')
			.replace(/ ['`’] (?=\S)/g, 'ʼ')
			.replace(/\(/g, '&#40');
	}

	public convertAlphabetOnly(text: string) {
		const noFixArr: string[] = [];
		return finalize(
			afterJoin(
				restoreBraces(
					applyNoFix(
						noFixArr,
						convertAlphabet(
							this.prepare(text, noFixArr, '<', false),
							this.general.abc
						)
					)
				)
			),
			'\n'
		);
	}

	private process(text: string): {
		splittedOrig: string[];
		splitted: string[];
	} {
		const { abc, j } = this.general;

		const splittedOrig = convertAlphabet(text, abc).split(' ');

		text = this.taraskevize(text.toLowerCase());
		if (j && abc !== ALPHABET.LATIN_JI)
			text = replaceIbyJ(text, j === REPLACE_J.ALWAYS);
		text = replaceWithDict(text, letters[abc]);

		let splitted = text.split(' ');
		if (abc !== ALPHABET.ARABIC) splitted = restoreCase(splitted, splittedOrig);
		return { splittedOrig, splitted };
	}

	protected taraskevize(text: string) {
		text = replaceWithDict(replaceWithDict(text, wordlist), noSoften);
		softening: do {
			text = replaceWithDict(text, softeners);
			for (const [pattern, result] of softeners)
				if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
			break;
		} while (true);

		return replaceWithDict(text.replace(/\ue0ff/g, ''), afterTarask);
	}
}
