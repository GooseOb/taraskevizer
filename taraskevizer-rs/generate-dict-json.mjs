// Generates JSON dictionary data for the Rust rewrite.
// Reads the compiled TS dictionaries (either RegExp or string patterns)
// and outputs {p, x, r} format.

import { dicts } from '../dist/index.js';

function getSource(pattern) {
	if (pattern instanceof RegExp) return pattern.source;
	return pattern;
}

const getPostfix = (start, count) => {
	count += start;
	let result = '';
	while (start < count) {
		result += '$' + start++;
	}
	return result;
};

const convert = (rawWl) => {
	const wl = [];
	for (let [pat, res] of rawWl) {
		let src = getSource(pat);
		if (src.includes('(?=')) {
			let count = 0;
			src = src.replace(/\(\?=/g, () => {
				++count;
				return '(';
			});

			if (res.startsWith('($&')) {
				res =
					res.replace(
						/^\(\$&/,
						'(' + src.replace(/\(.*?\)$/g, '').replace(/\[(.).*?]/g, '$1')
					) + getPostfix(1, count);
			} else {
				res += getPostfix(
					res
						.matchAll(/\$(\d)/g)
						.reduce((acc, item) => Math.max(item[1], acc), 0) + 1,
					count
				);
			}
		}
		if (res.includes('$&')) {
			if (/[()[\]?]/.test(src)) {
				src = `(${src})`;
				res = res
					.replace(/\$(\d)/g, (_$0, $1) => '$' + (+$1 + 1))
					.replace(/\$&/g, '$1');
			} else {
				res = res.replace(/\$&/g, src);
			}
		}
		if (src.includes('(?!')) {
			src = src.replace(/\(\?!\[/g, '([^').replace(/\(\?!(.)\)/g, '([^$1])');

			if (res.startsWith('($&')) {
				res = res.replace(/^\(\$&/, '(' + src.replace(/\(.*?\)$/g, '')) + '$1';
			} else {
				res +=
					'$' +
					(res
						.matchAll(/\$(\d)/g)
						.reduce((acc, item) => Math.max(item[1], acc), 0) +
						1);
			}
		}

		if (src.endsWith('( )')) {
			src = src.replace(/\( \)$/, ' ');
			res = res.replace(/\$\d$/, ' ');
		}

		if (src.includes('\\1')) {
			const [, letters, rest] = /\(\[(.+)\]\)\\1(.*)/.exec(src);
			for (const letter of letters) {
				wl.push({
					p: letter + letter + rest,
					r: res
						.replaceAll('$1', letter)
						.replace(/\$(\d)/g, (_$0, $1) => '$' + ($1 - 1)),
				});
			}
		} else {
			wl.push({ p: src, r: res });
			if (src === 'ге([^ ])') wl.push({ p: src, r: res });
		}
	}
	return wl;
};

async function main() {
	const fs = await import('fs/promises');
	const path = await import('path');
	const outDir = path.resolve('crates/core/src/dict/data');
	await fs.mkdir(outDir, { recursive: true });

	// --- 1. Alphabets ---
	const alphabetsOut = {};
	for (const [key, abc] of Object.entries(dicts.alphabets)) {
		const lower = abc.lower.value.map(([re, result]) => ({
			p: getSource(re),
			r: result,
		}));
		const upper = abc.upper
			? abc.upper.value.map(([re, result]) => ({
					p: getSource(re),
					r: result,
				}))
			: undefined;
		alphabetsOut[key] = { lower, upper };
	}
	await fs.writeFile(
		path.join(outDir, 'alphabets.json'),
		JSON.stringify(alphabetsOut)
	);
	console.log('Wrote alphabets.json');

	// --- 2. Gobj ---
	const gobj = Object.entries(dicts.gobj).map(([k, v]) => ({
		p: k,
		r: v,
	}));
	await fs.writeFile(path.join(outDir, 'gobj.json'), JSON.stringify(gobj));
	console.log('Wrote gobj.json');

	// --- 3. Wordlist ---
	const rawWl = dicts.wordlist.value;
	const wl = convert(rawWl);

	await fs.writeFile(path.join(outDir, 'wordlist.json'), JSON.stringify(wl));
	console.log(`Wrote wordlist.json (${wl.length} entries)`);

	// --- 4. Softeners + NoSoften ---
	const noSoftenRaw = dicts.noSoften.value;
	const softenRaw = dicts.softeners.value;

	const soften = convert(noSoftenRaw.concat(softenRaw));

	// Combined: protectors first so they get lower PatternID priority
	await fs.writeFile(path.join(outDir, 'soften.json'), JSON.stringify(soften));
	console.log(`Wrote soften.json (${soften.length} entries)`);

	// --- 5. commonPhonetic ---
	if (dicts.commonPhonetic) {
		const cp = convert(dicts.commonPhonetic);
		await fs.writeFile(
			path.join(outDir, 'common_phonetic.json'),
			JSON.stringify(cp)
		);
		console.log(`Wrote common_phonetic.json (${cp.length} entries)`);
	}

	// --- 6. phonetic ---
	if (dicts.phonetic) {
		const ph = convert(dicts.phonetic.value);
		await fs.writeFile(path.join(outDir, 'phonetic.json'), JSON.stringify(ph));
		console.log(`Wrote phonetic.json (${ph.length} entries)`);
	}

	// --- 7. iaWords (не→ня, без→бяз) ---
	if (dicts.iaWords) {
		const ia = convert(dicts.iaWords.value);
		await fs.writeFile(path.join(outDir, 'iawords.json'), JSON.stringify(ia));
		console.log(`Wrote iawords.json (${ia.length} entries)`);
	}

	console.log('\nDone!');
}

main().catch(console.error);
