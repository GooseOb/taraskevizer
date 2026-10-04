#!/usr/bin/env node
/**
 * `tarask` launcher shipped with the npm package.
 *
 * No conversion happens here: this script locates the prebuilt Rust
 * executable for the current platform (downloaded on install from GitHub
 * releases, or fetched on first run) and execs it with the arguments
 * untouched. Run `tarask --help` for the full option list.
 */
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import {
	ensureCliBinary,
	getBinaryPath,
	getDownloadUrl,
	resolveCliTarget,
} from './cli-binary.js';

const target = resolveCliTarget();
if (target === null) {
	process.stderr.write(
		`tarask: unsupported platform ${process.platform}-${process.arch}. ` +
			'Download a native binary manually from https://github.com/GooseOb/taraskevizer/releases\n'
	);
	process.exit(1);
}

const moduleUrl = import.meta.url;
let binaryPath = getBinaryPath(moduleUrl, target);
if (!existsSync(binaryPath)) {
	process.stderr.write('tarask: downloading the native binary...\n');
	const downloaded = await ensureCliBinary(moduleUrl);
	if (downloaded === null) {
		process.stderr.write(
			'tarask: the native binary is missing and could not be downloaded.\n' +
				`Download ${target} from ${getDownloadUrl(moduleUrl, target)}\n` +
				`and place it at ${binaryPath}\n`
		);
		process.exit(1);
	}
	binaryPath = downloaded;
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
	stdio: 'inherit',
});
if (result.error) {
	process.stderr.write(
		`tarask: failed to run the native binary at ${binaryPath}: ${result.error.message}\n`
	);
	process.exit(1);
}
process.exit(result.status ?? 1);
