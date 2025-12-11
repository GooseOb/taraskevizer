/* eslint-disable */
var {
	parentPort,
	workerData: { argv, dirname },
} = require('node:worker_threads');
var { resolve } = require('node:path');
var { pipelines } = require(resolve(dirname, '..'));
var { parseArgs } = require(resolve(dirname, 'parse-args'));

var { mode, cfg } = parseArgs(argv);

parentPort.on('message', (chunk) => {
	parentPort.postMessage(pipelines[mode](chunk, cfg));
});
