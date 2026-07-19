---
title: Configuration
description: Every option of TaraskConfig and what it controls.
sidebar:
  order: 3
---

The [`TaraskConfig`](/taraskevizer/reference/classes/taraskconfig/) class
tunes how conversions behave. Construct it with a partial object; any option
you omit keeps its default value.

```js
import { TaraskConfig, dicts } from "taraskevizer";

const cfg = new TaraskConfig({
  abc: dicts.alphabets.cyrillic,
  j: "always",
  variations: "first",
  wrappers: null,
  g: true,
});
```

You can also spread it onto another config or onto the predefined
[`htmlConfigOptions`](/taraskevizer/reference/variables/htmlconfigoptions/):

```js
import { TaraskConfig, htmlConfigOptions } from "taraskevizer";

const cfg = new TaraskConfig({ ...htmlConfigOptions, j: "random" });
```

## Options

### `abc` — alphabet

`Alphabet` (default `alphabets.cyrillic`). The alphabet used for
transliteration. Predefined alphabets live in
[`dicts.alphabets`](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/variables/cyrillic/):
`cyrillic`, `latin`, `latinJi`, and `arabic`. An alphabet is an object with a
`lower` (and optionally `upper`) `CallableDict`; if `upper` is omitted the
alphabet is treated as case-insensitive. See
[Customizing](/taraskevizer/guides/customizing/) for building your own.

### `j` — і → й after vowels

`"never" | "random" | "always"` (default `"never"`). Controls when `і`(`i`)
is replaced by `й`(`j`) after a vowel.

| Value   | Example                  |
| ------- | ------------------------ |
| _(default)_ | `яна і ён`          |
| never   | `яна і ён`               |
| random  | `яна і ён` or `яна й ён` |
| always  | `яна й ён`               |

Has no effect when `abc` is `alphabets.latinJi` (that alphabet already encodes
the `і`/`ј` distinction).

### `g` — ґ ↔ г

`boolean` (default `true` for `TaraskConfig`; `false` in
`htmlConfigOptions`). When `true`, replaces ґ(`g`) by г(`h`) in the Cyrillic
alphabet. This option only matters for the Cyrillic alphabet.

| Value | Example     |
| ----- | ----------- |
| true  | Ґвалт ґвалт |
| false | Гвалт гвалт |

### `variations` — variable word parts

`"no" | "first" | "all"` (default `"all"`). When a word part has multiple
valid spellings, this chooses which to show.

| Value  | Example     |
| ------ | ----------- |
| no     | Гродна      |
| first  | Горадня     |
| all    | (Гродна\|Горадня) |

How variations render depends on the
[wrappers](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/wrappers/)
you choose (plain text vs. HTML `tarL` tags vs. ANSI color).

### `wrappers` — marking changed parts

`Partial<Wrappers> | null` (default `null`). Wraps the changed parts of the
output so you can highlight or annotate them. Predefined wrappers are in
[`wrappers`](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/wrappers/):
`html` (emits `tarF`/`tarL`/`tarH` tags) and `ansiColor` (emits ANSI
escape codes). Set to `null` to skip wrapping entirely. See
[HTML tags](/taraskevizer/guides/html-tags/) for the tag reference and the
interactive helpers.

### `doEscapeCapitalized` — protect capitals

`boolean` (default `true`). When `true`, capitalized runs (acronyms,
all-caps fragments) are left untouched so they are not mistakenly
taraskevized. Set to `false` only when you are certain the text has no
protected capitals — the `alphabetic` pipeline forces this off internally.

### `newLine` — newline replacement

`string` (default `"\n"`). The string that `"\n"` in the input is replaced
with. For HTML you typically set it to `"<br>"`.

### `leftAngleBracket` — `<` replacement

`string` (default `"<"`). The string that `"<"` in the input is replaced
with. For HTML set it to `"&lt"` (note: the predefined `htmlConfigOptions`
already does this) so user-supplied angle brackets are escaped.

### `noFixPlaceholder` — protected-part placeholder

`string` (default `" \ue0fe "`). The internal placeholder used while
extracting parts that should **not** be converted (text enclosed in `< >`, or
marked with [special syntax](/taraskevizer/guides/special-syntax/)). You
should only change this if the default value happens to appear in your input
text and causes conflicts.
