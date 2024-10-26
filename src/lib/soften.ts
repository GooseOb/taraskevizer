import { noSoften, softeners } from '../dict/softening';

export const soften = (text: string) => {
	text = noSoften(text);
	do {
		text = softeners(text);
	} while (softeners.value.some(({ 0: pattern }) => pattern.test(text)));
	return text.replace(/\ue0ff/g, '');
};
