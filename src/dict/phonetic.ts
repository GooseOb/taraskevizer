import type { Dict } from './types';

export const commonPhonetinc: Dict = [
	[/шся /g, 'сься '],
	[/здж/g, 'ждж'],
	[/ ([бд]|кнд|нот)р /, ' $1\ue0ffр '],
	[/(\S\S[дт])р /, '$1ар '],
];

export const phonetic: Dict = (
	[
		[/сш/g, 'шш'],
		[/[зс]ч/g, 'шч'],
		[/чц(?![ьіеюя])/g, 'цц'],
		[/шц(?=ы )/g, 'сц'],
	] as Dict
).concat(commonPhonetinc);
