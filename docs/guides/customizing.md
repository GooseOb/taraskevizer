---
title: Customizing
description: Build custom pipelines, alphabets, wrappers, and dictionaries.
sidebar:
  order: 6
---

Beyond the builtin pipelines and config, taraskevizer exposes the building
blocks so you can adapt it to your own orthography rules.

## Composing custom pipelines

A pipeline is an ordered list of **steps** run against a shared context. Use
[`lib.pipe`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/pipe/)
to turn a step array into a callable pipeline, or
[`lib.asyncPipe`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/asyncpipe/)
if your steps are asynchronous.

```js
import { lib, steps, TaraskConfig, dicts } from "taraskevizer";

const myPipeline = lib.pipe([
  steps.trim,
  steps.resolveSpecialSyntax,
  steps.prepare,
  steps.convertAlphabet,
  steps.finalize,
  steps.untrim,
]);

myPipeline("планета", new TaraskConfig({ abc: dicts.alphabets.latin }));
```

A step is just a function `(ctx) => void` (or `async`) that mutates
`ctx.text`. The full catalogue of steps is in the
[steps namespace](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/).
To base a pipeline on a builtin one while depending less on its internal
order, `map`/`flatMap` over `pipeline.steps` and re-wrap with `lib.pipe`.

You can also write your own step:

```js
const myStep = (ctx) => {
  ctx.text = ctx.text.replace(/foo/g, "bar");
};
```

## Custom alphabets

An [`Alphabet`](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/type-aliases/alphabet/)
is `{ lower: CallableDict, upper?: CallableDict }`. A
[`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)
is a function (built from a list of `[pattern, replacement]` pairs via
[`callableDict`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/callabledict/))
that also exposes its source pairs on `.value`, so you can tweak it later.

```js
import { lib } from "taraskevizer";

const myAbc = {
  lower: lib.callableDict([
    [/а/g, "a"],
    [/б/g, "b"],
  ]),
};
```

Use [`copyDict`](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/copydict/)
to clone a dictionary before mutating it, so you don't change the original.

The predefined alphabets — `cyrillic`, `latin`, `latinJi`, `arabic` — live in
[`dicts.alphabets`](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/variables/cyrillic/).

## Custom wrappers

A [`Wrappers`](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/wrappers/)
object has three transformers: `fix` (wraps changed parts), `variable` (a
`VariationWrappers` keyed by `no`/`first`/`all`), and `letterH` (wraps the
ґ/г letter). The predefined `html` and `ansiColor` wrappers are good
starting points.

```js
import { wrappers } from "taraskevizer";

/** @type {import("taraskevizer").wrappers.Wrappers} */
const markdown = {
  fix: (c) => `**${c}**`,
  variable: {
    all: (c) => c,
    first: (c) => /^[^|]*?\|([^|)]*)/.exec(c)[1],
    no: (c) => /^\(([^|]*)/.exec(c)[1],
  },
  letterH: (c) => c,
};

// wrappers.html is a good starting point you can spread and override
const base = wrappers.html;
```

Pass your `Wrappers` object as the `wrappers` config option.

## Dictionaries

The conversion dictionaries exposed on
[`dicts`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/dict/) —
`wordlist`, `phonetic`, `softeners`, `noSoften`, `iaWords`, `iwords`, `gobj`,
and the `alphabets` — are plain `CallableDict` values you can reuse or extend
in your own steps. For example `lib.soften` and `lib.restoreCase` are small
reusable helpers built on top of them.
