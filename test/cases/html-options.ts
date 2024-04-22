import { HtmlOptions } from '../../src';

export default [
	[
		['жыццясцвярджальны план', {}],
		'жыц<tarF>ь</tarF>цяс<tarF>ь</tarF>ц<tarF>ь</tarF>вярджальны пл<tarF>я</tarF>н',
	],
	[['казахскі', {}], 'каз<tarF>ас</tarF>кі'],
	[['??????', {}], '??????'],
	[['газета', { g: false }], '<tarF><tarH>г</tarH></tarF>аз<tarF>э</tarF>та'],
	[['газета', { g: true }], '<tarF><tarH>ґ</tarH></tarF>аз<tarF>э</tarF>та'],
] satisfies [[string, Partial<HtmlOptions>], string][];
