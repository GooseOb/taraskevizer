import type { TaraskConfig } from '../../src';

export default [
	[
		['жыццясцвярджальны план', {}],
		'жыц<tarF>ьцясьць</tarF>вярджальны пл<tarF>я</tarF>н',
	],
	[['<,Планета>', {}], '&ltПл<tarF>я</tarF>н<tarF>э</tarF>та>'],
	[['&lt <,Планета>', {}], '&lt &ltПл<tarF>я</tarF>н<tarF>э</tarF>та>'],
	// [['казахскі', {}], 'каз<tarF>ас</tarF>кі'],
	[['??????', {}], '??????'],
	[['газета', { g: false }], '<tarH>г</tarH>аз<tarF>э</tarF>та'],
	[['газета', { g: true }], '<tarH>ґ</tarH>аз<tarF>э</tarF>та'],
] satisfies [[string, Partial<TaraskConfig>], string][];
