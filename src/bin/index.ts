#!/usr/bin/env node
import { cpus } from 'node:os';
import { Worker } from 'node:worker_threads';
import { pipelines } from '..';
import { parseArgs } from './parse-args';

declare const __CLI_HELP__: string;
declare const __CLI_PREFIX__: string;
declare const __VERSION__: string;
declare const __WORKER_CODE__: string;

const getPrint = (stream: NodeJS.WriteStream) => (msg: string) => {
	stream.write(__CLI_PREFIX__ + ' ' + msg);
};
const getPrintLn = (printFn: (msg: string) => void) => (msg: string) => {
	printFn(msg + '\n');
};

const print = getPrint(process.stdout);
const printErr = getPrint(process.stderr);
const printLn = getPrintLn(print);
const printErrLn = getPrintLn(printErr);

// AI-written, may need improvements
const splitIntoChunks = (text: string, n: number): string[] => {
	const size = Math.ceil(text.length / n);
	const chunks: string[] = [];

	let start = 0;

	for (let i = 0; i < n; i++) {
		let end = start + size;

		// Adjust end to the nearest newline character
		if (end < text.length) {
			const forward = text.indexOf('\n', end);
			const backward = text.lastIndexOf('\n', end);

			if (forward === -1 && backward === -1) {
				// no-op
			} else if (forward === -1) {
				end = backward;
			} else if (backward === -1) {
				end = forward;
			} else {
				end = forward - end < end - backward ? forward : backward;
			}
		}

		// Ensure we don't split inside HTML tags or special syntax tags
		if (end < text.length) {
			const lastOpen = text.lastIndexOf('<', end);
			const lastClose = text.lastIndexOf('>', end);

			if (lastOpen > lastClose) {
				const nextClose = text.indexOf('>', end);
				if (nextClose !== -1) {
					end = nextClose + 1;
				}
			}
		}

		if (end > text.length) end = text.length;

		chunks.push(text.slice(start, end));
		start = end;

		if (start >= text.length) break;
	}

	return chunks;
};

const getPrettyByteSize = (n: number) =>
	n < 1024
		? `${n} B`
		: n < 1024 * 1024
			? `${(n / 1024).toFixed(2)} KB`
			: `${(n / (1024 * 1024)).toFixed(2)} MB`;

process.argv.splice(0, 2);

const firstArg = process.argv[0];

if (firstArg) {
	if (firstArg === '-v' || firstArg === '--version') {
		printLn(__VERSION__);
		process.exit(0);
	}

	if (firstArg === '-h' || firstArg === '--help') {
		printLn(__CLI_HELP__);
		process.exit(0);
	}
}

const argv = process.argv.slice();
const { mode, cfg, doForceSingleThread } = parseArgs(process.argv);

const workers = {
	size: cpus()?.length || 1,
	workers: null as null | Worker[],
	init() {
		if (this.workers) return;
		process.stderr.write(`(Initializing ${this.size} workers... `);
		this.workers = Array.from(
			{ length: this.size },
			() =>
				new Worker(__WORKER_CODE__, {
					eval: true,
					workerData: { argv },
				})
		);
		process.stderr.write('done.) ');
	},
	process(chunks: string[]) {
		return Promise.all(
			chunks.map(
				(chunk, i) =>
					new Promise((resolve, reject) => {
						const worker = this.workers![i % this.size];

						worker.postMessage(chunk);

						worker.once('message', resolve);
						worker.once('error', reject);
					})
			)
		);
	},
};

const processText = async (text: string) => {
	let result = '';

	if (!doForceSingleThread && workers.size > 1 && text.length > 50_000) {
		workers.init();
		const chunks = splitIntoChunks(text, workers.size);

		const results = await workers.process(chunks);

		result = results.join('');
	} else {
		result = pipelines[mode](text, cfg);
	}

	if (!process.stdout.write(result)) {
		process.stdout.once('drain', () => {
			printErrLn('Drain event fired, exiting.');
			process.exit(0);
		});
	}
};

if (process.argv.length) {
	printErrLn('Processing the rest of command-line arguments as text...');
	await processText(process.argv.reverse().join(' '));
} else {
	const chunks: Uint8Array[] = [];
	let value = '';
	let length = 0;
	let byteLength = 0;
	const MAX_BYTE_LENGTH = 64 * 1024 * 1024;

	const getChunksString = () => Buffer.concat(chunks, length).toString();

	if (process.stdin.isTTY) {
		printErrLn('Enter the text');
		for await (const chunk of process.stdin) {
			chunks.push(chunk);
			length += chunk.length;
			if (chunk.includes('\n')) break;
		}

		await processText(getChunksString());
		process.stdout.write('\n');
	} else {
		printErrLn('Reading from stdin...');

		const processTextWithLogs = async (value: string) => {
			printErr(
				`Processing ${value.length
					.toString()
					.replace(
						/\B(?=(\d{3})+(?!\d))/g,
						' '
					)} characters (${getPrettyByteSize(Buffer.byteLength(value))}) chunk... `
			);
			const startTime = performance.now();
			await processText(value);
			process.stderr.write(
				`done in ${((performance.now() - startTime) / 1000).toFixed(2)} seconds.\n`
			);
		};

		for await (const chunk of process.stdin) {
			byteLength += chunk.byteLength;
			if (byteLength >= MAX_BYTE_LENGTH) {
				value += getChunksString();
				const lastNewlineIndex = value.lastIndexOf('\n');
				if (lastNewlineIndex === -1) {
					printErrLn(
						'\nInput exceeded maximum size of ' +
							MAX_BYTE_LENGTH +
							' bytes without a newline. Stopping.'
					);
					process.exit(1);
				}
				let valueForNextBatch = value.slice(lastNewlineIndex + 1);
				value = value.slice(0, lastNewlineIndex + 1);
				const lastOpeningTagIndex = value.lastIndexOf('<');
				const lastClosingTagIndex = value.lastIndexOf('>');
				if (
					lastOpeningTagIndex !== -1 &&
					lastOpeningTagIndex > lastClosingTagIndex
				) {
					const incompleteTag = value.slice(lastOpeningTagIndex);
					value = value.slice(0, lastOpeningTagIndex);
					valueForNextBatch = incompleteTag + valueForNextBatch;
				}
				await processTextWithLogs(value);
				value = valueForNextBatch;
				byteLength = chunk.byteLength;
				length = 0;
				chunks.length = 0;
			}
			length += chunk.length;
			chunks.push(chunk);
		}
		await processTextWithLogs(value + getChunksString());
	}
}

process.exit(0);
