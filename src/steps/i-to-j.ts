import { alphabets } from '../dict';
import type { TaraskStep } from './types';

const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

export const replaceIbyJ: TaraskStep = (options) => {
	const { abc, j } = options.cfg;
	if (j !== 'never' && abc !== alphabets.latinJi)
		options.text = options.text.replace(
			/(?<=[аеёіоуыэюя] )і (ў?)/g,
			j === 'always'
				? (_$0, $1) => toJ($1)
				: ($0, $1) => (Math.random() >= 0.5 ? toJ($1) : $0)
		);
};
