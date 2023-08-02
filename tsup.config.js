import { defineConfig } from 'tsup';
import path from 'path';
import { readFile, writeFile } from 'fs/promises';
import postprocess from './postprocess';
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
	async onSuccess() {
		for (const ext of ['js', 'mjs']) {
			const filePath = path.resolve(this.outDir, 'index.' + ext);
			readFile(filePath, 'utf8').then((text) =>
				writeFile(filePath, postprocess(text, ext))
			);
		}
	},
});
