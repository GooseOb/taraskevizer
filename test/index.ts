import { benchmark, print, startTestProcess } from './lib';
import { tarask, ALPHABET } from '../src';
import * as cases from './cases';
import * as debug from '../src/tools.debug';

const { endTestProcess, test } = startTestProcess();

test(
	'Taraskevization',
	(text) =>
		tarask(text, {
			OVERRIDE_toTarask(text, replaceWithDict, wordlist, softers, afterTarask) {
				text = debug.replaceWithDict(text, wordlist, /рыму/);
				softening: do {
					text = replaceWithDict(text, softers);
					for (const [pattern, result] of softers)
						if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
					break;
				} while (true);

				return replaceWithDict(text, afterTarask);
			},
		}),
	cases.taraskevization
);

test(
	'HtmlOptions',
	([text, html]) => tarask(text, { html }),
	cases.htmlOptions
);
test(
	'NonHtmlOptions',
	([text, nonHtml]) => tarask(text, { nonHtml }),
	cases.nonHtmlOptions
);

test('i to j', ([text, j, abc]) => tarask(text, { j, abc }), cases.itoj);

test('greek th', (text) => tarask(text, { abc: ALPHABET.GREEK }), cases.greek);

const code = endTestProcess();

if (process.argv.includes('--benchmark')) {
	const {
		existsSync,
		promises: { readFile },
	} = await import('node:fs');
	const path = 'test/large-text.txt';

	if (!existsSync(path)) {
		print('benchmark', path + ': no such a file', '36');
		process.exit(1);
	}
	const text = await readFile(path, 'utf8');
	benchmark('Taraskevization', () => {
		tarask(text, { nonHtml: true });
	});
}

process.exit(code);
