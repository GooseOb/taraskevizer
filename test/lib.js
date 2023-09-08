import { format } from 'util';

const colorize = (str, colorCode) => `\x1b[${colorCode}m${str}\x1b[0m`;
export const print = (label, msg, colorCode = '0') => {
	process.stdout.write(colorize(`[${label}] `, colorCode) + msg + '\n');
};

const colorizeCharInStr = (str, charIndex, colorCode) =>
	str.slice(0, charIndex) +
	colorize(str[charIndex], colorCode) +
	str.slice(charIndex + 1);

export const getTestProcess = () => {
	const summary = {
		passed: 0,
		failed: 0,
	};
	return {
		summary,
		test(name, fn, cases) {
			for (const [input, expectedValue] of cases) {
				const output = fn(input);
				if (output !== expectedValue) {
					let charIndex = 0;
					while (output[charIndex] === expectedValue[charIndex]) ++charIndex;
					const coloredOutput = colorizeCharInStr(output, charIndex, '31');
					const coloredExpected = colorizeCharInStr(
						expectedValue,
						charIndex,
						'32'
					);
					++summary.failed;
					print(
						'failed',
						`${name}:\ninput:    ${format(input)}\noutput:   ${format(
							coloredOutput
						)}\nexpected: ${format(coloredExpected)}`,
						'31'
					);
					return;
				}
			}
			++summary.passed;
			print('passed', name, '32');
		},
	};
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

export const benchmark = (name, fn, { showLogs = true, repeat = 5 } = {}) => {
	const printBenchmark = getBenchmarkPrinter(name);
	printBenchmark('running');
	const logs = [];
	const unhookStdout = hookStdout((data) => {
		logs.push(data);
	});
	const results = Array(repeat);
	for (let i = 0; i < repeat; i++) {
		const startTimeStamp = performance.now();
		fn();
		results[i] = performance.now() - startTimeStamp;
	}
	unhookStdout();
	process.stdout.moveCursor(0, -1);
	let min = results[0];
	let max = min;
	let sum = 0;
	for (const result of results) {
		if (result > max) max = result;
		else if (result < min) min = result;
		sum += result;
	}
	printBenchmark(
		`average result: ${sum / repeat} ms (min: ${min} ms, max: ${max} ms)`
	);
	if (showLogs && logs.length) process.stdout.write(logs.join(''));
};
