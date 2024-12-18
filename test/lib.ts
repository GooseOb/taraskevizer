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

export const startTestProcess = ({ long = false } = {}) => {
	print('test', 'start', '35');
	const passed: string[] = [];
	const failed: string[] = [];
	return {
		endTestProcess() {
			for (const output of passed) print('passed', output, '32');
			for (const output of failed) print('failed', output, '31');
			print('test', `${passed.length} passed, ${failed.length} failed`, '35');
			return failed.length ? 1 : 0;
		},
		test<TInput, TOutput extends string>(
			name: string,
			fn: (arg: TInput) => TOutput,
			cases: readonly (readonly [TInput, TOutput])[]
		) {
			const longArr = [];
			for (const { 0: input, 1: expectedValue } of cases) {
				let output: TOutput;
				try {
					output = fn(input);
				} catch (e) {
					(e as Error).cause = { name, input };
					throw e;
				}
				if (output !== expectedValue) {
					let charIndex = 0;
					while (output[charIndex] === expectedValue[charIndex]) ++charIndex;
					const coloredOutput = colorizeCharInStr(output, charIndex, '31');
					const coloredExpected = colorizeCharInStr(
						expectedValue,
						charIndex,
						'32'
					);
					failed.push(
						`${name}:\ninput:    "${format(input)}"\noutput:   "${format(
							coloredOutput
						)}"\nexpected: "${format(coloredExpected)}"`
					);
					return;
				} else if (long) {
					longArr.push(`${input} \x1b[36m->\x1b[0m ${output}`);
				}
			}
			passed.push(name + (long ? '\n' + longArr.join('\n') : ''));
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
