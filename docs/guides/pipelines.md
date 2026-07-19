---
title: Builtin pipelines
description: The conversion pipelines shipped with taraskevizer.
sidebar:
  order: 2
---

A pipeline is a preconfigured conversion routine. Pass text (and an
optional [`TaraskConfig`](/taraskevizer/reference/classes/taraskconfig/))
to it.

| Pipeline   | Taraskevization | Phonetization     | Alphabet | Special syntax |
| ---------- | --------------- | ----------------- | -------- | -------------- |
| tarask     | ✅              | ❌                | ✅       | ✅             |
| alphabetic | ❌              | ❌                | ✅       | ✅             |
| phonetic   | ❌              | ✅ (experimental) | ✅       | ✅             |

```js
import { pipelines } from "taraskevizer";

pipelines.tarask("планета"); // "плянэта"
pipelines.alphabetic("яна і іншыя"); // latin/alphabet-only transform
pipelines.phonetic("планета"); // experimental phonetic transform
```

## What each pipeline does

### `tarask`

The full taraskevization (classical-orthography) pipeline. It lowercases the
text, applies the alphabet conversion, replaces `і` after vowels
according to `j`, converts the alphabet, restores the original capitalization,
highlights the differences, escapes angle brackets, and finally applies
variations and wrappers. This is the one most users want.

### `alphabetic`

Changes **only the alphabet** — it does not taraskevize spelling. It forces
`doEscapeCapitalized` to `false` for the duration of the run, so acronyms and
all-caps fragments are converted letter-for-letter without the heuristics that
protect them in `tarask`. Use it when you already have text in the academic
orthography and only need the alphabet transliteration (e.g. Cyrillic ↔ Latin).

### `phonetic` (experimental)

Like `tarask`, but additionally applies phoneticization and the `і`/`ј`
iotacization step. The phonetic layer is experimental and may change.

## How a pipeline is built

Internally a pipeline is just an ordered list of **steps**. Each step receives
a shared context (`{ text, cfg, storage }`) and mutates `text` in place. You
can inspect the steps of any builtin pipeline:

```js
import { pipelines } from "taraskevizer";

pipelines.tarask.steps; // array of step functions
```

To build on a builtin pipeline without depending on its exact internal order,
`map` (to replace steps) or `flatMap` (to add/remove steps) over
`pipeline.steps` and feed the result back into
[`pipe`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/pipe/):

```js
import { pipelines, lib, steps } from "taraskevizer";

const myFirstStep = steps.trim;

const myTarask = lib.pipe(
  pipelines.tarask.steps.map((step) =>
    step === pipelines.tarask.steps[0] ? myFirstStep : step,
  ),
);
```

For full control, compose your own pipeline from the individual
[steps](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/) using
[`lib.pipe`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/pipe/)
(synchronous) or
[`lib.asyncPipe`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/asyncpipe/)
(asynchronous). See [Customizing](/taraskevizer/guides/customizing/).
