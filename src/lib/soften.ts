import { noSoften, softeners } from '../dict/softening';
import { replaceWithDict } from './replace-with-dict';

export const soften = (text: string) => {
	text = replaceWithDict(text, noSoften);
	do {
		text = replaceWithDict(text, softeners);
	} while (softeners.some(([pattern]) => pattern.test(text)));
	return text.replace(/\ue0ff/g, '');
};
