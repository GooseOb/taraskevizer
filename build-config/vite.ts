import * as path from 'path';
import { fileURLToPath } from 'url';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';

const rootPath = path.resolve(
	path.dirname(fileURLToPath(import.meta.url)),
	'../'
);
const srcPath = path.resolve(rootPath, 'src');

export default defineConfig({
	plugins: [
		dts({
			insertTypesEntry: true,
			include: [srcPath],
		}),
	],
	build: {
		lib: {
			entry: path.resolve(rootPath, 'src/index.ts'),
			name: 'my-lib',
			formats: ['es', 'cjs'],
			fileName: 'index',
		},
		outDir: path.resolve(rootPath, 'dist'),
	},
});
