import * as path from 'path';
import { readFile, unlink, writeFile } from 'fs/promises';
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

export default defineConfig({
	...common,
	clean: true,
	entry: ['src/index.ts'],
	format: ['cjs', 'esm'],
	dts: true,
	esbuildPlugins: [noDebugFiles],
	async onSuccess() {
		if (!isDeploy) generateJSON();
		await build({
			...common,
			entry: [path.resolve(root, 'src/bin.ts')],
			bundle: false,
			format: 'esm',
			async onSuccess() {
				await fixCharsIn('bin.js');
			},
		});
		for (const ext of ['js', 'cjs']) await fixCharsIn('index.' + ext);
		setTimeout(() => unlink(path.resolve(outDir, 'index.d.cts')), 1500);
	},
});
