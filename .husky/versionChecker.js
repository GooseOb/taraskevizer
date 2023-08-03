import { exec as execSync } from 'node:child_process';
import { promisify } from 'node:util';
import pjson from '../package.json' assert { type: 'json' };

const isCISkipped = /\[(skip ci|ci skip)]/i.test(process.argv[1]);
if (isCISkipped) process.exit(0);

const exec = promisify(execSync);

const { stdout } = await exec('git show main:package.json');
const version = JSON.parse(stdout).version;

if (version === pjson.version)
	throw new Error(
		"\x1b[31mVersion isn't changed. Add \x1b[32m[skip ci]\x1b[31m to the commit message to commit without triggering CI pipeline\x1b[0m",
	);
