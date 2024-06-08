import * as path from 'path';
import { readFile, writeFile } from 'fs/promises';
import { defineConfig, build, Options } from 'tsup';
import postprocess from './postprocess';
import generateJSON from './json-generator';
import noDebugFiles from './esbuild-plugins/no-debug-files';
import { fileURLToPath } from 'url';

const isDeploy = process.env.BUILD_MODE?.toLowerCase() === 'deployment';

const root = path.resolve(fileURLToPath(import.meta.url), '..', '..');
const outDir = path.resolve(root, 'dist');

const common = {
	sourcemap: false,
	splitting: false,
	minify: false,
	outDir,
} satisfies Options;

const fixCharsIn = (fileName: string) => {
	const filePath = path.resolve(outDir, fileName);
	return readFile(filePath, 'utf8').then((text) =>
		writeFile(filePath, postprocess(text))
	);
};

const define = {
	__VERSION__: `"${JSON.parse(await readFile(path.resolve(root, 'package.json'), 'utf8')).version}"`,
	__CLI_HELP__: JSON.stringify(
		(await readFile(path.resolve(root, 'cli-help.txt'), 'utf8'))
			.replace(/<fix>/g, '\x1b[32m')
			.replace(/<\/fix>/g, '\x1b[0m')
			.replace(/\[taraskevizer]/g, '\x1b[34m[taraskevizer]\x1b[0m')
	),
};

export default defineConfig({
	...common,
	clean: true,
	entry: ['src/index.ts'],
	format: ['cjs', 'esm'],
	dts: true,
	esbuildOptions: (options) => {
		options.legalComments = 'inline';
	},
	esbuildPlugins: [noDebugFiles],
	async onSuccess() {
		if (!isDeploy) generateJSON();
		await Promise.all([
			build({
				...common,
				define,
				entry: [path.resolve(root, 'src/bin.ts')],
				bundle: false,
				format: 'esm',
				async onSuccess() {
					await fixCharsIn('bin.js');
				},
			}),
			['js', 'cjs'].map((ext) => fixCharsIn('index.' + ext)),
		]);
	},
});
