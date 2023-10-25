import { TaraskOptions } from '../../src';

export default [
	[['жыццясцвярджальны\nплан'], 'жыцьцясьцьвярджальны\nплян'],
	[['жыццясцвярджальны\r\nплан'], 'жыцьцясьцьвярджальны\r\nплян'],
	[['жыццясцвярджальны\r\n\t\t\tплан'], 'жыцьцясьцьвярджальны\r\n\t\t\tплян'],
	[
		['жыццясцвярджальны\n\t\t\tплан', { html: true }],
		'жыц<tarF>ьцясьць</tarF>вярджальны<br>\t\t\tпл<tarF>я</tarF>н',
	],
] satisfies [[string, TaraskOptions?], string][] as [
	[string, TaraskOptions | undefined],
	string
][];
