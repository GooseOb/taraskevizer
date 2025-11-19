#!/usr/bin/env node
import { cpus } from 'node:os';
import { Worker } from 'node:worker_threads';
import { pipelines } from '..';
import { parseArgs } from './parse-args';

declare const __CLI_HELP__: string;
declare const __CLI_PREFIX__: string;
declare const __VERSION__: string;

const printWithPrefix = (msg: string) => {
	process.stdout.write(__CLI_PREFIX__ + ' ' + msg + '\n');
};

const splitIntoChunks = (text: string, n: number): string[] => {
	const size = Math.ceil(text.length / n);
	const chunks: string[] = [];

	let start = 0;

	for (let i = 0; i < n; i++) {
		let end = start + size;

		if (end < text.length) {
			const forward = text.indexOf('\n', end);
			const backward = text.lastIndexOf('\n', end);

			if (forward === -1 && backward === -1) {
				// no-op, use raw end
			} else if (forward === -1) {
				end = backward;
			} else if (backward === -1) {
				end = forward;
			} else {
				end = forward - end < end - backward ? forward : backward;
			}
		}

		if (end > text.length) end = text.length;
		chunks.push(text.slice(start, end));
		start = end;
		if (start >= text.length) break;
	}

	return chunks;
};

process.argv.splice(0, 2);

const firstArg = process.argv[0];

if (firstArg) {
	if (firstArg === '-v' || firstArg === '--version') {
		printWithPrefix(__VERSION__);
		process.exit(0);
	}

	if (firstArg === '-h' || firstArg === '--help') {
		printWithPrefix(__CLI_HELP__);
		process.exit(0);
	}
}

const { mode, cfg, doForceSingleThread } = parseArgs(process.argv);

let text = '';
if (process.argv.length) {
	text = process.argv.reverse().join(' ');
} else {
	const chunks: Uint8Array[] = [];
	let length = 0;

	if (process.stdin.isTTY) {
		printWithPrefix('Enter the text');
		for await (const chunk of process.stdin) {
			chunks.push(chunk);
			length += chunk.length;
			if (chunk.includes('\n')) break;
		}
	} else {
		for await (const chunk of process.stdin) {
			chunks.push(chunk);
			length += chunk.length;
		}
	}

	text = Buffer.concat(chunks, length).toString();
}

let result = '';

if (text.length > 50_000 && !doForceSingleThread) {
	const cpuCount = Math.max(1, cpus()?.length || 1);
	const chunks = splitIntoChunks(text, cpuCount);

	const WORKER_CODE = `
const { parentPort, workerData } = require('node:worker_threads');
const { pipelines } = require('./dist');
const { parseArgs } = require('./dist/bin/parse-args');
const { argv, chunk } = workerData;
const { mode, cfg } = parseArgs(argv);
parentPort.postMessage(pipelines[mode](chunk, cfg));`;
	const results = await Promise.all(
		chunks.map(
			(chunk) =>
				new Promise((resolve, reject) => {
					const worker = new Worker(WORKER_CODE, {
						eval: true,
						workerData: { argv: process.argv, chunk },
					});

					worker.on('message', resolve);
					worker.on('error', reject);
					worker.on('exit', (code) => {
						if (code !== 0) reject(new Error('Worker exit code ' + code));
					});
				})
		)
	);

	result = results.join('\n') + '\n';
} else {
	result = pipelines[mode](text, cfg) + '\n';
}

if (process.stdout.write(result)) {
	process.exit(0);
} else {
	process.stdout.once('drain', () => {
		process.exit(0);
	});
}
