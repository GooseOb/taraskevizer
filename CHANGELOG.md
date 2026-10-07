# Changelog

All notable user-facing changes to taraskevizer are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This file is the source of truth for GitHub release notes: the `Publish`
workflow extracts the section matching the tag and prepends it to the release
body. `generate_release_notes` stays on, so GitHub still appends the
auto-generated compare link below it.

## [11.0.4]

### Fixed

- Wordlist batches were not independent. Batches now are smaller but independent so they can be safely combined in one pattern.

### Changed

- "яўрэі" -> "(яў|габ)рэі" instead of "габрэі"

### Added

- "ласо" -> "лясо"

## [11.0.3]

### Fixed

- Chunking no longer treats an escaped `\>` as closing an angle-bracket
  tag, so cuts stay suppressed until a real `>` (e.g. inside `<math>` spans
  using TeX spacing like `\>`).
- Chunking no longer has a 1MB limit on the size of a chunk

## [11.0.2]

### Added

- New `splitIntoChunks(text, chunks)` export (JS/WASM) for parallel
  conversion in `Worker`s: splits text into worker-ready `string[]` chunks
  using the same boundaries the native CLI feeds to rayon. Convert each
  chunk with `tarask` / `alphabetic` / `phonetic` (each worker with its own
  WASM instance) and concatenate the results in order.

## [11.0.1]

### Changed

`TaraskConfig` and `htmlConfigOptions` are now managed by TS, not WASM. So you don't have to call `await init()` before using them.

## [11.0.0]

### JSON BREAKING CHANGES

- JSON files are now shipped as `json.zip`, not flat in release.
- `wordlist.json` is now `[pattern: string, result: string][][]` instead of `[pattern: string, result: string][]`. Entries that can be combined into a signle pattern are grouped into a sub-array.

### Added

- Conversion core rewritten in Rust and shipped to JavaScript as WebAssembly.
- Prebuilt native `tarask` CLI binaries attached to every release: Linux
  (x64, ARM64), macOS (Intel, Apple Silicon) and Windows (x64).
- The npm package installs a `tarask` command that runs the native binary
  for the current platform (downloaded automatically on install or on first
  run).
- `--not-escape-caps` is accepted as an alias of `--no-escape-caps`.

### Changed

- The JS API is now a thin typed layer over the WASM core: top-level
  `tarask` / `alphabetic` / `phonetic` functions instead of the `pipelines`
  namespace. The default entry needs one `await init()` call; the `node`
  export condition initializes from the bundled binary on import, and the
  browser bundle (`dist/bundle.js`) is fully synchronous.
- `TaraskConfig` options use plain string values (`abc: 'latin'`,
  `wrappers: 'none' | 'html' | 'ansi'`); arbitrary callback wrappers can no
  longer cross the WASM boundary. `htmlConfigOptions` is now a function
  returning the preset.
- Diff highlighting is now more percise, and does not highlight parts that are handled by the `variations` step
  e.g. `с[ь]мяе[сь]ся`, not `с[ьмяесь]ся`, `у (а|ва)кне`, not `у [(а|ва)]кне`

### Removed

- The pure-TypeScript pipeline internals (`dicts`, `steps`, `pipelines`,
  `lib`, `wrappers` and `alphabets` namespace imports) and the
  JavaScript implementation of the CLI.

### Performance

- On 30MB slice of Wikipedia dump, the Rust CLI is 6-10x faster than the JS CLI. (parallel enabled for both)
- If used as a JS library (WASM module), might be slower on small inputs due to the overhead of crossing the WASM boundary.
- Conversion of 10MB Wikipedia slice in browser is 3-4x faster than the JS implementation.

[11.0.0]: https://github.com/GooseOb/taraskevizer/compare/v10.4.24...v11.0.0
