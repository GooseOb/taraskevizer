import { getLabel, testOnCases } from './lib';
import { ALPHABET, J, tarask, TaraskOptions, taraskSync } from '../../src';

testOnCases('\x1b[31mTaraskevization', taraskSync, [
	['жыццясцвярджальны план', 'жыцьцясьцьвярджальны плян'],
	['варэнне', 'варэньне'],
	['секцыя селекцыйных селектараў', 'сэкцыя сэлекцыйных сэлектараў'],
	['эскалатар эскалацыі вайберу', 'эскалятар эскаляцыі вайбэру'],
]);

testOnCases(
	'\x1b[32mSync and async functions return the same result',
	taraskSync,
	[['жыццясцвярджальны план', await tarask('жыццясцвярджальны план')]]
);

testOnCases(
	'\x1b[33mHtmlOptions',
	([text, html]) => taraskSync(text, { html }),
	[
		[
			['жыццясцвярджальны план', {}],
			'жыц<tarF>ьцясьць</tarF>вярджальны пл<tarF>я</tarF>н',
		],
		[['газета', { g: false }], '<tarH>г</tarH>аз<tarF>э</tarF>та'],
		[['газета', { g: true }], '<tarH>ґ</tarH>аз<tarF>э</tarF>та'],
	] satisfies [[string, TaraskOptions['html']], string][],
	getLabel('j')
);

testOnCases(
	'\x1b[34mi -> j',
	([text, j, abc]) => taraskSync(text, { j, abc }),
	[
		[['Яна і ён'], 'Яна і ён'],
		[['Яна і ён', J.NEVER], 'Яна і ён'],
		[['Яна і ён', J.ALWAYS], 'Яна й ён'],
		[['Яна і ён', undefined, ALPHABET.LATIN], 'Jana i jon'],
		[['Яна і ён', J.NEVER, ALPHABET.LATIN], 'Jana i jon'],
		[['Яна і ён', J.ALWAYS, ALPHABET.LATIN], 'Jana j jon'],
	] satisfies [[string, TaraskOptions['j']?, TaraskOptions['abc']?], string][],
	getLabel('j')
);

testOnCases(
	'\x1b[36mgreek th',
	(text) => taraskSync(text, { abc: ALPHABET.GREEK }),
	[
		['матэматычная кафедра', 'μαθεματίτσ̌ναυα καθεδρα'],
		['афінская арыфметыка', 'αθενσκαυα αρίθμετίκα'],
		['Фесалонікі, Салонікі', '(Θεσ|Σ)αλυονικι, (Σ|Θεσ)αλυονικι'],
		['Тэарэма Піфагора', 'Θεαρεμα Πιθαγορα'],
		['Тэатр арыфметычнага тэрматэізму', 'Θεαταρ αρίθμετίτσ̌ναγα θερμαθειζμϋ'],
		[
			'Этычны тэзіс Томаса, Марты і Агаты',
			'Εθίτσ̌ν(ί|αυα) θε(ζισ|ζα) Θομασα, Μαρθί ι Αγαθί',
		],
		['Сінтэтычны трамбозавы лабірынт', 'Σίνθετίτσ̌νί θραμμποζαβί λυαμπιρίνθ'],
		[
			'Апатычны псіхапат на троне з паталогіяй',
			'Απαθίτσ̌νί πσίχαπαθ να θρονη ζ παθαλυογιυαυ',
		],
		[
			'Антрапагенны апафеоз лагарыфмаў з апафемы',
			'Ανθραπαγκηννί απαθεοζ λυαγαρίθμαΰ ζ απαθεμί',
		],
		['Базіраваць', 'Μπαζαβατσ’'],
	]
);
