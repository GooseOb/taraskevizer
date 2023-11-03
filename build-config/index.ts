import * as path from 'path';
import { readFile, writeFile } from 'fs/promises';
import { defineConfig, Options } from 'tsup';
import postprocess from './postprocess';
import generateJSON from './json-generator';
import noDebugFiles from './esbuild-plugins/no-debug-files';

const isDeploy = process.env.BUILD_MODE?.toLowerCase() === 'deployment';

export default defineConfig({
	entry: ['src/index.ts'],
	clean: true,
	sourcemap: false,
	splitting: false,
	minify: false,
	format: ['cjs', 'esm'],
	dts: true,
	esbuildPlugins: [noDebugFiles],
	outDir: 'dist',
	onSuccess(this: Required<Options>) {
		if (!isDeploy) generateJSON();
		for (const ext of ['js', 'cjs']) {
			const filePath = path.resolve(this.outDir, 'index.' + ext);
			readFile(filePath, 'utf8').then((text) =>
				writeFile(filePath, postprocess(text))
			);
		}
		return new Promise((res) => {
			res();
		});
	},
});
