---
editUrl: false
next: true
prev: true
title: "splitIntoChunks"
---

> **splitIntoChunks**(`text`, `chunks`): `string`[]

Defined in: [index.ts:74](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/index.ts#L74)

Split text into worker-ready chunks for parallel conversion.

Same boundaries the native CLI feeds to rayon: cuts after a spacing char
at/after `len / chunks` bytes, never inside `<…>` tags or after
apostrophe-likes, capped at 1 MiB per chunk. Returns owned strings (not
byte offsets) because Rust byte offsets don't map to JS UTF-16 indices.

Convert each chunk (in `Worker`s, each with its own WASM instance) and
concatenate the results in order:

```js
const chunks = splitIntoChunks(bigText, navigator.hardwareConcurrency);
const out = (await Promise.all(chunks.map((c) => convertInWorker(c)))).join('');
```

## Parameters

### text

`string`

### chunks

`number`

## Returns

`string`[]
