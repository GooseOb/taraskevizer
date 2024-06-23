import type { Dict } from './types';

export const commonPhonetinc: Dict = [
	[/шся /g, 'сься '],
	[/здж/g, 'ждж'],
];

export const phonetic: Dict = (
	[
		[/сш/g, 'шш'],
		[/[зс]ч/g, 'шч'],
		[/чц/g, 'цц'],
		[/шц(?=ы )/g, 'сц'],
	] as Dict
).concat(commonPhonetinc);
