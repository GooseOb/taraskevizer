import { noSoften, softeners, wordlist } from '../dict';
import { afterTarask, replaceWithDict } from '../lib';
import type { TaraskStep } from './types';

const wordlistPlusNoSoften = wordlist.concat(noSoften);

export const taraskevize: TaraskStep = (options) => {
	let { text } = options;
	text = replaceWithDict(text, wordlistPlusNoSoften);
	softening: do {
		text = replaceWithDict(text, softeners);
		for (const [pattern, result] of softeners)
			if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
		break;
	} while (true);

	options.text = replaceWithDict(text.replace(/\ue0ff/g, ''), afterTarask);
};
