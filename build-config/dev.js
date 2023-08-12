import { defineConfig } from 'tsup';
import { exec } from 'node:child_process';
import pjson from '../package.json' assert { type: 'json' };

export default defineConfig({
	entry: ['src/index.ts'],
	format: ['esm'],
	onSuccess() {
		exec(pjson.scripts.test, (err, stdout, stderr) => {
			if (stdout) console.log(stdout);
			if (stderr) console.log(stderr);
		});
	},
});
