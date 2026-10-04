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

## Fixed entry points

The pipelines are fixed entry points into the compiled core: they cannot be
introspected, recomposed, or extended from JavaScript. Everything tunable
lives in the config — see [Configuration](/taraskevizer/guides/configuration/).
Per-word control without a custom config is possible via
[Special syntax](/taraskevizer/guides/special-syntax/).
