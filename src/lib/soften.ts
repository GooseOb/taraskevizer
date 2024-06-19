import { softeners } from '../dict/softening';
import { replaceWithDict } from './replace-with-dict';

export const soften = (text: string) => {
	softening: do {
		text = replaceWithDict(text, softeners);
		for (const [pattern, result] of softeners)
			if (result !== '$1дзьдз' && pattern.test(text)) continue softening;
		return text;
	} while (true);
};
