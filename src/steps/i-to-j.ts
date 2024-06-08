import { alphabets } from '../dict';
import { REPLACE_J } from '../constants';
import type { TaraskStep } from './types';

const toJ = (shortU: '' | 'ў') => 'й ' + (shortU ? 'у' : '');

export const replaceIbyJ: TaraskStep = (options) => {
	const { abc, j } = options.cfg.general;
	if (j && abc !== alphabets.latinJi)
		options.text = options.text.replace(
			/(?<=[аеёіоуыэюя] )і (ў?)/g,
			j === REPLACE_J.ALWAYS
				? ($0, $1) => toJ($1)
				: ($0, $1) => (Math.random() >= 0.5 ? toJ($1) : $0)
		);
};
