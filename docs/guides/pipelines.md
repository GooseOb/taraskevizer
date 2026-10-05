---
title: Builtin pipelines
description: The conversion pipelines shipped with taraskevizer.
sidebar:
  order: 2
---

A pipeline is an exported conversion function. Pass text (and an optional
[`TaraskConfig`](/taraskevizer/reference/classes/taraskconfig/) or plain
option object) to it.

| Pipeline   | Taraskevization | Phonetization     | Alphabet | Special syntax |
| ---------- | --------------- | ----------------- | -------- | -------------- |
| tarask     | ✅              | ❌                | ✅       | ✅             |
| alphabetic | ❌              | ❌                | ✅       | ✅             |
| phonetic   | ❌              | ✅ (experimental) | ✅       | ✅             |

```js
import { init, tarask, alphabetic, phonetic } from "taraskevizer";

await init(); // skip under Node.js: the `node` export initializes automatically

tarask("планета"); // "плянэта"
alphabetic("яна і іншыя", { abc: "latin" }); // "jana i inšyja"
phonetic("планета"); // experimental phonetic transform
```

## What each pipeline does

### `tarask`

The full taraskevization (classical-orthography) pipeline. It lowercases the
text, applies the alphabet conversion, replaces `і` after vowels
according to `j`, converts the alphabet, restores the original capitalization,
highlights the differences, escapes angle brackets, and finally applies
variations and wrappers. This is the one most users want.

### `alphabetic`

Changes **only the alphabet** — it does not taraskevize spelling. Everything
is transliterated letter-for-letter, including capitalized words: the
`doEscapeCapitalized` protection only matters for `tarask`. Use it when you
already have text in the academic orthography and only need the alphabet
transliteration (e.g. Cyrillic ↔ Latin).

### `phonetic` (experimental)

Applies assimilative palatalization, phoneticization and the `і`/`ј`
iotacization step. The phonetic layer is experimental and may change.

## Converting large texts in parallel

The WASM pipelines are single-threaded: one `tarask()` call uses one core.
For large inputs, split the text with
[`splitIntoChunks`](/taraskevizer/reference/functions/splitIntoChunks/),
convert each chunk on its own worker (each worker with its own WASM
instance), and join the results in order:

```js
// main.js
import { splitIntoChunks } from "taraskevizer";

// Sends one chunk to a fresh worker running worker.js below and resolves
// with the converted result. One worker per chunk is fine here because the
// chunk count matches the core count.
const convertInWorker = (chunk, config) =>
  new Promise((resolve, reject) => {
    const worker = new Worker(new URL("./worker.js", import.meta.url), {
      type: "module",
    });
    worker.onmessage = ({ data }) => {
      resolve(data);
      worker.terminate();
    };
    worker.onerror = reject;
    worker.postMessage({ chunk, config });
  });

const chunks = splitIntoChunks(bigText, navigator.hardwareConcurrency);
const out = (
  await Promise.all(chunks.map((chunk) => convertInWorker(chunk, config)))
).join("");
```

```js
// worker.js
import init, { tarask } from "taraskevizer";

await init(); // skip with the browser bundle: it is fully synchronous
onmessage = ({ data: { chunk, config } }) => {
  postMessage(tarask(chunk, config));
};
```

Pass the config as a plain option object (`{ abc: "latin" }`) so it
survives the `postMessage` round-trip. Chunk boundaries are word-safe
(never inside `<…>` tags or after apostrophe-likes), so chunk-by-chunk
conversion matches a single call — as long as every worker uses the same
config and chunks are joined in order. The worker overhead isn't worth it
for short texts; as a rule of thumb, parallelize above ~1 MB.

If you can shell out instead, the native
[`tarask` CLI](/taraskevizer/guides/cli/) already does all of this for
you, including streaming multi-GB inputs.

## Fixed entry points

The pipelines are fixed entry points into the compiled core: they cannot be
introspected, recomposed, or extended from JavaScript. Everything tunable
lives in the config — see [Configuration](/taraskevizer/guides/configuration/).
Per-word control without a custom config is possible via
[Special syntax](/taraskevizer/guides/special-syntax/).
