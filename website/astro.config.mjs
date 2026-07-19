// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightTypeDoc from 'starlight-typedoc';

// https://astro.build/config
export default defineConfig({
	site: 'https://gooseob.github.io',
	base: '/taraskevizer',
	integrations: [
		starlight({
			title: 'taraskevizer',
			description: 'Канвэртацыя акадэмічнага правапісу ў клясычны',
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/GooseOb/taraskevizer',
				},
			],
			plugins: [
				// Generate the API reference from the TypeScript source into docs/reference.
				starlightTypeDoc({
					entryPoints: ['../src/index.ts'],
					tsconfig: '../tsconfig.json',
					output: 'reference',
					pagination: true,
					sidebar: { label: 'Reference', collapsed: false },
				}),
			],
			sidebar: [
				{
					label: 'Guides',
					items: [{ autogenerate: { directory: 'guides' } }],
				},
				{
					label: 'Reference',
					items: [
						{ autogenerate: { directory: 'reference', collapsed: false } },
					],
				},
			],
		}),
	],
});
