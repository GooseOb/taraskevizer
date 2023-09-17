import { format } from 'util';

const colorize = (str: string, colorCode: `${number}`) =>
	`\x1b[${colorCode}m${str}\x1b[0m`;
export const print = (
	label: string,
	msg: string,
	colorCode: `${number}` = '0'
) => {
	process.stdout.write(colorize(`[${label}] `, colorCode) + msg + '\n');
};

const colorizeCharInStr = (
	str: string,
	charIndex: number,
	colorCode: `${number}`
) =>
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
		test<TInput, TOutput extends string>(
			name: string,
			fn: (arg: TInput) => TOutput,
			cases: readonly (readonly [TInput, TOutput])[]
		) {
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

const getBenchmarkPrinter = (name: string) => (msg: string) => {
	print('benchmark', name + ', ' + msg, '36');
};

const hookStdout = (callback: typeof process.stdout.write) => {
	const { write } = process.stdout;
	process.stdout.write = callback;
	return () => {
		process.stdout.write = write;
	};
};

export const benchmark = (
	name: string,
	fn: () => void,
	{ showLogs = true, repeat = 5 } = {}
) => {
	const printBenchmark = getBenchmarkPrinter(name);
	printBenchmark('running');
	const logs: (string | Uint8Array)[] = [];
	const unhookStdout = hookStdout((data) => {
		logs.push(data);
		return true;
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
