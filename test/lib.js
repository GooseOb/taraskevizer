const colorize = (str, colorCode) => `\x1b[${colorCode}m${str}\x1b[0m`;
export const print = (label, msg, colorCode = '0') => {
	console.log(colorize(`[${label}] `, colorCode) + msg);
};
const failed = (testName, msg) => {
	print('failed', testName + ': ' + msg, '31');
	process.exit(1);
};
const passed = (testName) => {
	print('passed', testName, '32');
};

const colorizeCharInStr = (str, charIndex, colorCode) =>
	str.slice(0, charIndex) +
	colorize(str[charIndex], colorCode) +
	str.slice(charIndex + 1);

export const test = (name, fn, cases) => {
	for (const [input, expectedValue] of cases) {
		const output = fn(input);
		if (output !== expectedValue) {
			let charIndex = 0;
			while (output[charIndex] === expectedValue[charIndex]) ++charIndex;
			const coloredOutput = colorizeCharInStr(output, charIndex, '31');
			const coloredExpected = colorizeCharInStr(expectedValue, charIndex, '32');
			failed(
				name,
				`\ninput:    ${input}\noutput:   ${coloredOutput}\nexpected: ${coloredExpected}`
			);
		}
	}
	passed(name);
};

const getBenchmarkPrinter = (name) => (msg) => {
	print('benchmark', name + ', ' + msg, '36');
};

const hookStdout = (callback) => {
	const { write } = process.stdout;
	process.stdout.write = callback;
	return () => {
		process.stdout.write = write;
	};
};

export const benchmark = (name, fn, showLogs = true) => {
	const printBenchmark = getBenchmarkPrinter(name);
	printBenchmark('running');
	const logs = [];
	const unhookStdout = hookStdout((data) => {
		logs.push(data);
	});
	const startTimeStamp = performance.now();
	fn();
	const benchmarkResult = performance.now() - startTimeStamp;
	unhookStdout();
	process.stdout.moveCursor(0, -1);
	printBenchmark(`result: ${benchmarkResult} ms`);
	if (showLogs && logs.length) console.log(logs.join(''));
};
