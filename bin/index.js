#!/usr/bin/env node

const print = (...msgs) => {
	console.log('[taraskevizer]', ...msgs);
};

if (process.argv[2] === '-v' || process.argv[2] === '--version') {
	print(require('../package.json').version);
	process.exit(0);
}

const { taraskSync } = require('../dist');
const text = process.argv.splice(2).join(' ');

print(taraskSync(text));
