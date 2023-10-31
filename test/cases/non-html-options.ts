import { NonHtmlOptions, VARIATION } from '../../src';

export default [
	[['Гродна', { variations: VARIATION.NO }], 'Гродна'],
	[['Гродна', { variations: VARIATION.FIRST }], 'Горадня'],
	[['Гродна', { variations: VARIATION.ALL }], '(Гродна|Горадня)'],
] as [[string, NonHtmlOptions], string][];
