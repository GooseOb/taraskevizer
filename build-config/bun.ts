import { readFileSync, writeFileSync } from 'fs';
import noDebugFiles from './bun-plugins/no-debug-files';
import generateJsonFiles from './json-generator';
import postprocess from './postprocess';

// TODO: add d.ts files generation

generateJsonFiles();

Bun.build({
	entrypoints: ['./src/index.ts'],
	outdir: 'dist-bun',
	plugins: [noDebugFiles],
	format: 'esm', // TODO: add cjs when it'll be available
}).then((output) => {
	const { path } = output.outputs[0];
	writeFileSync(path, postprocess(readFileSync(path, 'utf-8')));
});
