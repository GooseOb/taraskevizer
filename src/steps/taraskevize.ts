import { noSoften, wordlist } from '../dict/index';
import {
	afterTarask,
	mutatingStep,
	replaceWithDict,
	soften,
} from '../lib/index';

const wordlistPlusNoSoften = wordlist.concat(noSoften);

export const taraskevize = mutatingStep(({ text }) =>
	replaceWithDict(
		soften(replaceWithDict(text, wordlistPlusNoSoften))
			.replace(/\ue0ff/g, '')
			.replace(/не пра/g, 'не&nbsp;пра'),
		afterTarask
	).replace(/не&nbsp;пра/g, 'не пра')
);
