export const print = (
	label: string,
	msg: string,
	colorCode: `${number}` = '0'
) => {
	console.log(`\x1b[${colorCode}m[${label}]\x1b[0m ${msg}`);
};
const fail = (testName: string, msg: string) => {
	print('fail', `${testName}: ${msg}`, '31');
	process.exit(1);
};
const passed = (testName: string) => {
	print('passed', testName, '32');
};

export const test = <T>(
	name: string,
	fn: (arg: T) => T,
	cases: [source: T, expectedResult: T][]
) => {
	for (const [input, expectedValue] of cases) {
		const output = fn(input);
		if (output !== expectedValue)
			fail(
				name,
				`\ninput: ${input}\noutput: ${output}\nexpected: ${expectedValue}`
			);
	}
	passed(name);
};
