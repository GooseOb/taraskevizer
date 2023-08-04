export const print = (label, msg, colorCode = '0') => {
	console.log(`\x1b[${colorCode}m[${label}]\x1b[0m ${msg}`);
};
const fail = (testName, msg) => {
	print('fail', `${testName}: ${msg}`, '31');
	process.exit(1);
};
const passed = (testName) => {
	print('passed', testName, '32');
};

export const test = (name, fn, cases) => {
	for (const [input, expectedValue] of cases) {
		const output = fn(input);
		if (output !== expectedValue)
			fail(
				name,
				`\ninput: ${input}\noutput: ${output}\nexpected: ${expectedValue}`,
			);
	}
	passed(name);
};
