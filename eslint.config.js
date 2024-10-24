import globals from 'globals';
import pluginJs from '@eslint/js';
import tseslint from 'typescript-eslint';

export default [
	{ files: ['**/*.{js,mjs,cjs,ts}'] },
	{ languageOptions: { globals: globals.browser } },
	{ languageOptions: { globals: globals.node } },
	pluginJs.configs.recommended,
	...tseslint.configs.recommended,
	{
		rules: {
			'no-misleading-character-class': ['error', { allowEscape: true }],
			'@typescript-eslint/no-explicit-any': 'off',
		},
	},
	{ ignores: ['dist/**', 'docs/**'] },
];
