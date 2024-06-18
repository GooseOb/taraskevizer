import { noSoften, softeners, wordlist } from '../dict/index';
import { afterTarask, mutatingStep, replaceWithDict } from '../lib/index';

const wordlistPlusNoSoften = wordlist.concat(noSoften);

export const taraskevize = mutatingStep(({ text }) => {
	text = replaceWithDict(text, wordlistPlusNoSoften);
	softening: do {
		text = replaceWithDict(text, softeners);
		for (const [pattern, result] of softeners)
			if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
		break;
	} while (true);

	return replaceWithDict(
		text.replace(/\ue0ff/g, '').replace(/не пра/g, 'не&nbsp;пра'),
		afterTarask
	).replace(/не&nbsp;пра/g, 'не пра');
});
