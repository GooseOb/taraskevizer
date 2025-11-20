/* eslint-disable */
const { parentPort, workerData } = require('node:worker_threads');
const { pipelines } = require('./dist');
const { parseArgs } = require('./dist/bin/parse-args');

const { mode, cfg } = parseArgs(workerData.argv);

parentPort.on('message', (chunk) => {
	parentPort.postMessage(pipelines[mode](chunk, cfg));
});
