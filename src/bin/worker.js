/* eslint-disable */
const {
	parentPort,
	workerData: { argv, dirname },
} = require('node:worker_threads');
const { resolve } = require('node:path');
const { pipelines } = require(resolve(dirname, '..'));
const { parseArgs } = require(resolve(dirname, 'parse-args'));

const { mode, cfg } = parseArgs(argv);

parentPort.on('message', (chunk) => {
	parentPort.postMessage(pipelines[mode](chunk, cfg));
});
