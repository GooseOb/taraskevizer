import {
	dicts,
	htmlConfigOptions,
	pipelines,
	TaraskConfig,
	wrappers,
} from '..';

const toHashTable = (
	dict: readonly (readonly [readonly string[], () => void])[]
) => {
	const result: Record<string, () => void> = {};
	for (const { 0: options, 1: callback } of dict)
		for (const option of options) result[option] = callback;
	return result;
};

export const parseArgs = (argv: string[]) => {
	let cfg: Partial<TaraskConfig> = {
		g: true,
		variations: 'all',
		wrappers: wrappers.ansiColor,
	};

	let mode: keyof typeof pipelines = 'tarask';

	let isHtml = false;

	let doForceSingleThread = false;

	const optionDict = toHashTable([
		[
			['--latin', '-l'],
			() => {
				cfg.abc = dicts.alphabets.latin;
			},
		],
		[
			['--latin-ji', '-lj'],
			() => {
				cfg.abc = dicts.alphabets.latinJi;
			},
		],
		[
			['--arabic', '-a'],
			() => {
				cfg.abc = dicts.alphabets.arabic;
			},
		],
		[
			['--jrandom', '-jr'],
			() => {
				cfg.j = 'random';
			},
		],
		[
			['--jalways', '-ja'],
			() => {
				cfg.j = 'always';
			},
		],
		[
			['--no-escape-caps', '-nec'],
			() => {
				cfg.doEscapeCapitalized = false;
			},
		],
		[
			['--h'],
			() => {
				cfg.g = false;
			},
		],
		[
			['--no-variations', '-nv'],
			() => {
				cfg.variations = 'no';
			},
		],
		[
			['--first-variation', '-fv'],
			() => {
				cfg.variations = 'first';
			},
		],
		[
			['--no-color', '-nc'],
			() => {
				cfg.wrappers = null;
			},
		],
		[
			['--html', '-html'],
			() => {
				isHtml = true;
				cfg.wrappers = htmlConfigOptions.wrappers;
			},
		],
		[
			['--alphabet-only', '-abc'],
			() => {
				mode = 'alphabetic';
			},
		],
		[
			['--phonetic', '-ph'],
			() => {
				mode = 'phonetic';
			},
		],
		[
			['--single-thread', '-st'],
			() => {
				doForceSingleThread = true;
			},
		],
	]);

	let currOption: string | undefined;

	argv.reverse();
	while ((currOption = argv.pop())) {
		if (currOption in optionDict) {
			optionDict[currOption]();
		} else {
			argv.push(currOption);
			break;
		}
	}

	cfg = new TaraskConfig(
		isHtml
			? {
					...htmlConfigOptions,
					...cfg,
				}
			: cfg
	);

	return { cfg, mode, doForceSingleThread };
};
