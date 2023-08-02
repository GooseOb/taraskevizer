export default {
	name: 'no-debug-files',
	setup(build) {
		build.onLoad({ filter: /(^|\.)debug(\.|$)/ }, () => {
			throw new Error('debug files should not be used in production build');
		});
	},
};
