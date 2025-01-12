import { format } from 'util';

const colorize = (str: string, colorCode: `${number}`) =>
	`\x1b[${colorCode}m${str}\x1b[0m`;

const print = (label: string, msg: string, colorCode: `${number}` = '0') => {
	process.stdout.write(colorize(`[${label}] `, colorCode) + msg + '\n');
};
print.start = () => {
	print('test', 'start', '35');
};
print.final = (passed: number, failed: number) => {
	print('test', `${passed} passed, ${failed} failed`, '35');
};
print.passed = (msg: string) => {
	print('passed', msg, '32');
};
print.failed = (msg: string) => {
	print('failed', msg, '31');
};
print.inProgress = (name: string) => {
	print('      ', name, '36');
};
print.benchmark = (msg: string) => {
	print('benchmark', msg, '36');
};

const initFailed = () => {
	const arr: string[] = [];
	return {
		print: () => {
			arr.forEach(print.failed);
		},
		add: ({
			name,
			input,
			output,
			expectedValue,
		}: {
			name: string;
			input: any;
			output: string;
			expectedValue: string;
		}) => {
			let charIndex = 0;
			while (output[charIndex] === expectedValue[charIndex]) ++charIndex;

			const coloredOutput = colorizeCharInStr(output, charIndex, '31');
			const coloredExpected = colorizeCharInStr(expectedValue, charIndex, '32');
			arr.push(
				`${name}:\ninput:    "${format(input)}"\noutput:   "${format(
					coloredOutput
				)}"\nexpected: "${format(coloredExpected)}"`
			);
		},
		count: () => arr.length,
	};
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
	print.start();
	let passedCount = 0;
	const failed = initFailed();
	return {
		endTestProcess() {
			failed.print();
			print.final(passedCount, failed.count());
			return failed.count() ? 1 : 0;
		},
		test<TInput, TOutput extends string>(
			name: string,
			fn: (arg: TInput) => TOutput,
			cases: readonly (readonly [TInput, TOutput])[]
		) {
			if (process.stdout.moveCursor) {
				print.inProgress(name);
				process.stdout.moveCursor(0, -1);
			}
			let longStr = '';
			for (const { 0: input, 1: expectedValue } of cases) {
				let output: TOutput;
				try {
					output = fn(input);
				} catch (e) {
					(e as Error).cause = { name, input };
					throw e;
				}
				if (output !== expectedValue) {
					if (long) {
						longStr += `\n${input} \x1b[31m->\x1b[0m ${output}`;
					}
					print.failed(name + longStr);
					failed.add({ name, input, output, expectedValue });
					return;
				} else if (long) {
					longStr += `\n${input} \x1b[36m->\x1b[0m ${output}`;
				}
			}
			++passedCount;
			print.passed(name + longStr);
		},
	};
};

const getBenchmarkPrinter = (name: string) => (msg: string) => {
	print.benchmark(name + ', ' + msg);
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
