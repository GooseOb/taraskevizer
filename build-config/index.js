import path from 'path';
import { readFile, writeFile } from 'fs/promises';
import { defineConfig } from 'tsup';
import postprocess from './postprocess';
import generateJSON from './json-generator';
import noDebugFiles from './esbuild-plugins/no-debug-files';

export default defineConfig({
	entry: ['src/index.ts'],
	clean: true,
	sourcemap: false,
	splitting: false,
	minify: false,
	format: ['cjs', 'esm'],
	dts: true,
	esbuildPlugins: [noDebugFiles],
	onSuccess() {
		generateJSON();
		for (const ext of ['js', 'cjs']) {
			const filePath = path.resolve(this.outDir, 'index.' + ext);
			readFile(filePath, 'utf8').then((text) =>
				writeFile(filePath, postprocess(text, ext))
			);
		}
	},
});
