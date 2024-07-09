import { TaraskConfig } from '../../src';

export default [
	[['Гродна', { variations: 'no' }], 'Гродна'],
	[['Гродна', { variations: 'first' }], 'Горадня'],
	[['Гродна', { variations: 'all' }], '(Гродна|Горадня)'],
] as [[string, Partial<TaraskConfig>], string][];
