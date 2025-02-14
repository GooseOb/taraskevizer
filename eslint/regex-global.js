const requireGlobalFlag = {
	meta: {
		type: 'problem',
		docs: {
			description:
				'Ensure all regular expressions in specified files have the global (g) flag',
		},
		messages: {
			missingGlobalFlag:
				'Regular expressions in this file must have the global (g) flag.',
		},
		schema: [
			{
				type: 'object',
				properties: {
					pathTest: {
						type: 'regexp',
					},
				},
				additionalProperties: false,
			},
		],
	},

	create: (ctx) => {
		return ctx.options[0]?.pathTest?.test(ctx.getFilename())
			? {
					Literal(node) {
						if (node.regex && !node.regex.flags.includes('g')) {
							ctx.report({
								node,
								messageId: 'missingGlobalFlag',
							});
						}
					},

					NewExpression(node) {
						if (
							node.callee.name === 'RegExp' &&
							node.arguments.length > 1 &&
							node.arguments[1].type === 'Literal' &&
							typeof node.arguments[1].value === 'string' &&
							!node.arguments[1].value.includes('g')
						) {
							ctx.report({
								node,
								messageId: 'missingGlobalFlag',
							});
						}
					},
				}
			: {};
	},
};

export const regexGlobal = {
	plugins: {
		'regex-global': {
			rules: {
				'regex-global': requireGlobalFlag,
			},
		},
	},
};
