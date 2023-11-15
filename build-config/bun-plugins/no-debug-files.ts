import type { BunPlugin } from 'bun';

export default {
	name: 'no-debug-files',
	setup(build) {
		build.onLoad({ filter: /(^|\.)debug(\.|$)/ }, () => {
			process.stderr.write(
				'[no-debug-files] debug files should not be used in production build\n'
			);
			process.exit(1);
		});
	},
} satisfies BunPlugin;
