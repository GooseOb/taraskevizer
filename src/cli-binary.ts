/**
 * Shared plumbing for the `tarask` npm binaries.
 *
 * The npm package does not reimplement conversion in JavaScript. Instead,
 * `cli.ts` (the `tarask`/`taraskevizer` bin entries) locates the prebuilt
 * Rust executable for the current platform and spawns it, while
 * `install-cli-binary.ts` (the `postinstall` hook) downloads that
 * executable from GitHub releases ahead of time.
 *
 * Release asset naming (produced by `.github/workflows/main.yml`):
 * `tarask-<rust-target-triple>[.exe]`, e.g.
 * `tarask-x86_64-unknown-linux-gnu` or `tarask-x86_64-pc-windows-msvc.exe`.
 */
import {
	chmodSync,
	existsSync,
	mkdirSync,
	readFileSync,
	writeFileSync,
} from 'node:fs';
import { get as getHttp } from 'node:http';
import type { IncomingMessage } from 'node:http';
import { get as getHttps } from 'node:https';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

export const BINARY_BASENAME = 'tarask';

/** Map the Node.js platform/arch to a release asset triple. Null = unsupported. */
export const resolveCliTarget = (): string | null => {
	const { platform, arch } = process;
	if (platform === 'linux' && arch === 'x64') {
		return 'x86_64-unknown-linux-gnu';
	}
	if (platform === 'linux' && arch === 'arm64') {
		return 'aarch64-unknown-linux-gnu';
	}
	if (platform === 'darwin' && arch === 'x64') {
		return 'x86_64-apple-darwin';
	}
	if (platform === 'darwin' && arch === 'arm64') {
		return 'aarch64-apple-darwin';
	}
	if (platform === 'win32' && arch === 'x64') {
		return 'x86_64-pc-windows-msvc';
	}
	return null;
};

const readOwnPackageJson = (
	moduleUrl: string | URL
): { version?: string; repository?: { url?: string } | string } => {
	try {
		return JSON.parse(
			readFileSync(new URL('../package.json', moduleUrl), 'utf8')
		) as { version?: string };
	} catch {
		return {};
	}
};

/** Version of the installed package; the release tag is `v<version>`. */
export const getPackageVersion = (moduleUrl: string | URL): string =>
	readOwnPackageJson(moduleUrl).version ?? 'unknown';

/** `owner/repo` parsed from package.json, with a hardcoded fallback. */
export const getRepoSlug = (moduleUrl: string | URL): string => {
	const repository = readOwnPackageJson(moduleUrl).repository;
	const url = typeof repository === 'string' ? repository : repository?.url;
	const match = url?.match(/github\.com[/:]([^/]+\/[^/]+?)(?:\.git)?$/);
	return match?.[1] ?? 'GooseOb/taraskevizer';
};

export const getAssetName = (target: string): string =>
	`${BINARY_BASENAME}-${target}${process.platform === 'win32' ? '.exe' : ''}`;

/** Absolute path of the native binary inside the installed package. */
export const getBinaryPath = (
	moduleUrl: string | URL,
	target: string
): string => fileURLToPath(new URL(`./bin/${getAssetName(target)}`, moduleUrl));

export const getDownloadUrl = (
	moduleUrl: string | URL,
	target: string
): string => {
	const version = getPackageVersion(moduleUrl);
	return `https://github.com/${getRepoSlug(moduleUrl)}/releases/download/v${version}/${getAssetName(target)}`;
};

const download = (url: string, maxRedirects = 5): Promise<Buffer> =>
	new Promise((resolve, reject) => {
		if (maxRedirects < 0) {
			reject(new Error(`too many redirects while downloading ${url}`));
			return;
		}
		const get = url.startsWith('http://') ? getHttp : getHttps;
		const req = get(
			url,
			{ headers: { 'User-Agent': 'taraskevizer-installer' } },
			(res: IncomingMessage) => {
				const { statusCode, headers } = res;
				if (
					statusCode !== null &&
					statusCode !== undefined &&
					statusCode >= 300 &&
					statusCode < 400 &&
					headers.location
				) {
					res.resume();
					resolve(
						download(
							new URL(headers.location, url).toString(),
							maxRedirects - 1
						)
					);
					return;
				}
				if (statusCode !== 200) {
					res.resume();
					reject(new Error(`download failed: HTTP ${statusCode} for ${url}`));
					return;
				}
				const chunks: Buffer[] = [];
				res.on('data', (chunk: Buffer) => chunks.push(chunk));
				res.on('end', () => resolve(Buffer.concat(chunks)));
				res.on('error', reject);
			}
		);
		req.on('error', reject);
		req.setTimeout(120_000, () => {
			req.destroy(new Error(`download timed out for ${url}`));
		});
	});

/**
 * Download the native binary into the package if it is missing.
 *
 * Never throws: installers and launchers report the returned null with
 * their own messaging. Returns the absolute binary path on success
 * (including when it was already present).
 */
export const ensureCliBinary = async (
	moduleUrl: string | URL
): Promise<string | null> => {
	const target = resolveCliTarget();
	if (!target) {
		return null;
	}
	const binaryPath = getBinaryPath(moduleUrl, target);
	if (existsSync(binaryPath)) {
		return binaryPath;
	}
	if (process.env.TARASKEVIZER_SKIP_BINARY_DOWNLOAD) {
		return null;
	}
	try {
		const data = await download(getDownloadUrl(moduleUrl, target));
		if (data.length === 0) {
			return null;
		}
		mkdirSync(dirname(binaryPath), { recursive: true });
		writeFileSync(binaryPath, data);
		if (process.platform !== 'win32') {
			chmodSync(binaryPath, 0o755);
		}
		return binaryPath;
	} catch {
		return null;
	}
};
