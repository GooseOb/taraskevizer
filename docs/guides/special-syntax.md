---
title: Special syntax
description: Inline markers to control how individual words are converted.
sidebar:
  order: 5
---

Special syntax lets you control the conversion of individual words directly in
the input text — without writing a custom config. It is honored by all three
builtin pipelines (`tarask`, `alphabetic`, `phonetic`).

## The three controls

|             | fix (convert normally) | no fix (leave as-is) | change only alphabet |
| ----------- | ---------------------- | -------------------- | -------------------- |
| brackets    | `<,Планета>`           | `<Планета>`          | `<*Планета>`         |
| no brackets | `Планета`              | `<.Планета>`         | `<*.Планета>`        |

By default a word **is** converted, so "fix" with brackets is only needed to
override a word that would otherwise be left alone. The three markers are:

- **`<.X>` / `<*.X>` — no fix.** Leave `X` untouched (don't convert it). With
  the leading `*` it still transliterates the alphabet but skips taraskevization.
- **`<,X>` — force fix.** Convert `X` even if it would normally be escaped.
- **`<*X>` — alphabet only.** Change only the alphabet of `X`, skipping
  taraskevization.

## Bracketed vs. unbracketed forms

The bracketed form `<…>` is the most explicit and works anywhere, including
mid-word. The unbracketed form uses a leading marker character and is handy
for whole words:

```js
import { pipelines, TaraskConfig, dicts } from "taraskevizer";

const cfg = new TaraskConfig({ abc: dicts.alphabets.latin });

pipelines.tarask("Гэта <Планета>", cfg);
// "Heta <Планета>"

pipelines.tarask("плянэта <.Планета>", cfg);
// "planeta Планета"

pipelines.tarask("<*Гродна>", cfg);
// <Hrodna>
```

## Escaping angle brackets

Inside the markers, a literal `>` is written as `\>` so it is not treated as
the marker's closing bracket:

```text
<тэкст з \> стрэлкай>
```

## How it works under the hood

Marked parts are extracted before conversion and stored, then re-inserted
afterwards using the `noFixPlaceholder`. The steps involved are
[`resolveSpecialSyntax`](/taraskevizer/reference/taraskevizer/namespaces/steps/variables/resolvespecialsyntax/)
(which captures the parts) and
[`applyNoFix`](/taraskevizer/reference/taraskevizer/namespaces/steps/variables/applynofix/)
(which puts them back). You normally never call these directly — they run as
part of every builtin pipeline.
