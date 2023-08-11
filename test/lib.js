export const print = (label, msg, colorCode = '0') => {
	console.log(`\x1b[${colorCode}m[${label}]\x1b[0m ${msg}`);
};
const failed = (testName, msg) => {
	print('failed', `${testName}: ${msg}`, '31');
	process.exit(1);
};
const passed = (testName) => {
	print('passed', testName, '32');
};

export const test = (name, fn, cases) => {
	for (const [input, expectedValue] of cases) {
		const output = fn(input);
		if (output !== expectedValue)
			failed(
				name,
				`\ninput: ${input}\noutput: ${output}\nexpected: ${expectedValue}`
			);
	}
	passed(name);
};

export const benchmark = async (name, fn) => {
	print('benchmark', `${name}, start`, '36');
	const startTimeStamp = performance.now();
	fn();
	const benchmarkResult = performance.now() - startTimeStamp;
	print('benchmark', `${name}, result: ${benchmarkResult} ms`, '36');
};
