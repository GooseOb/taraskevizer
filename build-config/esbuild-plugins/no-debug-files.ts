export default {
	name: 'no-debug-files',
	setup(build: any) {
		build.onLoad({ filter: /(^|\.)debug(\.|$)/ }, () => {
			throw new Error('debug files should not be used in production build');
		});
	},
};
