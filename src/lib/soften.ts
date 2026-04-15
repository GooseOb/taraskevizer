import { noSoften, softeners } from '@/dict/softening';

export const soften = (text: string) => {
	text = noSoften(text);
	let prev;
	do {
		prev = text;
		text = softeners(text);
	} while (prev !== text);
	return text.replace(/\ue0ff/g, '');
};
