import { TaraskConfig } from '../../src';

export default [
	[
		['жыццясцвярджальны план', {}],
		'жыц<tarF>ьцясьць</tarF>вярджальны пл<tarF>я</tarF>н',
	],
	// [['казахскі', {}], 'каз<tarF>ас</tarF>кі'],
	[['??????', {}], '??????'],
	[['газета', { g: false }], '<tarH>г</tarH>аз<tarF>э</tarF>та'],
	[['газета', { g: true }], '<tarH>ґ</tarH>аз<tarF>э</tarF>та'],
] satisfies [[string, Partial<TaraskConfig>], string][];
