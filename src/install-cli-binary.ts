/**
 * `postinstall` hook: download the prebuilt Rust `tarask` executable for the
 * current platform from GitHub releases into `./bin/` next to the launcher.
 *
 * This script must never fail the install: no network, an unpublished
 * version (e.g. developing on main before the release exists) or an
 * unsupported platform only produce a warning. The launcher retries the
 * download on first run instead. Set `TARASKEVIZER_SKIP_BINARY_DOWNLOAD`
 * to skip explicitly.
 */
import { existsSync } from 'node:fs';
import {
	ensureCliBinary,
	getBinaryPath,
	getDownloadUrl,
	resolveCliTarget,
} from './cli-binary.js';

const moduleUrl = import.meta.url;

try {
	const target = resolveCliTarget();
	if (target === null) {
		console.warn(
			`[taraskevizer] no prebuilt CLI binary for ${process.platform}-${process.arch}; see https://github.com/GooseOb/taraskevizer/releases`
		);
	} else if (existsSync(getBinaryPath(moduleUrl, target))) {
		console.log('[taraskevizer] CLI binary is already installed');
	} else {
		const binaryPath = await ensureCliBinary(moduleUrl);
		if (binaryPath === null) {
			console.warn(
				'[taraskevizer] could not download the CLI binary from ' +
					`${getDownloadUrl(moduleUrl, target)}; ` +
					'it will be fetched on first `tarask` run instead'
			);
		} else {
			console.log(`[taraskevizer] CLI binary installed at ${binaryPath}`);
		}
	}
} catch (error) {
	console.warn(
		`[taraskevizer] CLI binary setup skipped: ${(error as Error).message}`
	);
}
