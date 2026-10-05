---
editUrl: false
next: true
prev: true
title: "splitIntoChunks"
---

> **splitIntoChunks**(`text`, `chunks`): `string`[]

Defined in: [index.ts:74](https://github.com/GooseOb/taraskevizer/blob/a3736cae79c7c5ceaa665b3650c8158cdc5a65ae/src/index.ts#L74)

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
