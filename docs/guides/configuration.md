---
title: Configuration
description: Every option of TaraskConfig and what it controls.
sidebar:
  order: 3
---

The [`TaraskConfig`](/taraskevizer/reference/classes/taraskconfig/) class
tunes how conversions behave. Construct it with a partial object; any option
you omit keeps its default value. A plain
[`TaraskOptions`](/taraskevizer/reference/interfaces/taraskoptions/) object
works anywhere a config is accepted, no class instance needed.

```js
import { TaraskConfig } from "taraskevizer";

const cfg = new TaraskConfig({
  abc: "cyrillic",
  j: "always",
  variations: "first",
  wrappers: "none",
  g: true,
});
```

To tweak the predefined
[`htmlConfigOptions()`](/taraskevizer/reference/functions/htmlconfigoptions/),
mutate the returned instance:

```js
import { htmlConfigOptions } from "taraskevizer";

const cfg = htmlConfigOptions();
cfg.j = "random";
```

## Options

### `abc` — alphabet

[`TaraskAlphabet`](/taraskevizer/reference/type-aliases/taraskalphabet/)
(default `"cyrillic"`). The alphabet used for transliteration: `"cyrillic"`,
`"latin"`, `"latinJi"`, or `"arabic"`. Only the builtin alphabets are
available.

### `j` — і → й after vowels

[`TaraskJ`](/taraskevizer/reference/type-aliases/taraskj/) (default
`"never"`). Controls when `і`(`i`) is replaced by `й`(`j`) after a vowel.

| Value   | Example                  |
| ------- | ------------------------ |
| _(default)_ | `яна і ён`          |
| never   | `яна і ён`               |
| random  | `яна і ён` or `яна й ён` |
| always  | `яна й ён`               |

Has no effect when `abc` is `"latinJi"` (that alphabet already encodes the
`і`/`ј` distinction).

### `g` — ґ ↔ г

`boolean` (default `true`; `false` in `htmlConfigOptions()`). When `true`,
replaces ґ(`g`) by г(`h`) in the Cyrillic alphabet. This option only matters
for the Cyrillic alphabet.

| Value | Example     |
| ----- | ----------- |
| true  | Ґвалт ґвалт |
| false | Гвалт гвалт |

### `variations` — variable word parts

[`TaraskVariations`](/taraskevizer/reference/type-aliases/taraskvariations/)
(default `"all"`). When a word part has multiple valid spellings, this
chooses which to show.

| Value  | Example     |
| ------ | ----------- |
| no     | Гродна      |
| first  | Горадня     |
| all    | (Гродна\|Горадня) |

How variations render depends on the `wrappers` option below (plain text vs.
HTML `tarL` tags vs. ANSI color).

### `wrappers` — marking changed parts

[`TaraskWrappers`](/taraskevizer/reference/type-aliases/taraskwrappers/)
(default `"none"`). Wraps the changed parts of the output so you can
highlight or annotate them: `"html"` emits `tarF`/`tarL`/`tarH` tags and
`"ansi"` emits ANSI escape codes. Set to `"none"` to skip wrapping entirely.
Only these predefined sets are available — arbitrary wrapper functions
cannot cross into the compiled core. See
[HTML tags](/taraskevizer/guides/html-tags/) for the tag reference and the
interactive helpers.

### `doEscapeCapitalized` — protect capitals

`boolean` (default `true`). When `true`, capitalized runs (acronyms,
all-caps fragments) are left untouched so they are not mistakenly
taraskevized. Set to `false` only when you are certain the text has no
protected capitals. It has no effect on the `alphabetic` pipeline, which
transliterates everything letter-for-letter.

### `newLine` — newline replacement

`string` (default `"\n"`). The string that `"\n"` in the input is replaced
with. For HTML you typically set it to `"<br>"`.

### `leftAngleBracket` — `<` replacement

`string` (default `"<"`). The string that `"<"` in the input is replaced
with. For HTML set it to `"&lt"` (note: the predefined `htmlConfigOptions()`
already does this) so user-supplied angle brackets are escaped.

### `noFixPlaceholder` — protected-part placeholder

`string` (default `" \ue0fe "`). The internal placeholder used while
extracting parts that should **not** be converted (text enclosed in `< >`, or
marked with [special syntax](/taraskevizer/guides/special-syntax/)). You
should only change this if the default value happens to appear in your input
text and causes conflicts.
