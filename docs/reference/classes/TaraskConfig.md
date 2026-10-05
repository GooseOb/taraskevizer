---
editUrl: false
next: true
prev: true
title: "TaraskConfig"
---

Defined in: [config.ts:28](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L28)

Conversion options. Every field keeps its name (`doEscapeCapitalized`,
`newLine`, …) and default. Only predefined wrapper sets are available
(`wrappers: 'none' | 'html' | 'ansi'`).

```js
new TaraskConfig() // defaults
new TaraskConfig({ abc: 'latin', g: false })
```

## Constructors

### Constructor

> **new TaraskConfig**(`options?`): `TaraskConfig`

Defined in: [config.ts:54](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L54)

#### Parameters

##### options?

`Partial`\<`TaraskConfig`\> \| `null`

#### Returns

`TaraskConfig`

## Properties

### abc

> **abc**: [`TaraskAlphabet`](/taraskevizer/reference/type-aliases/taraskalphabet/) = `'cyrillic'`

Defined in: [config.ts:30](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L30)

Alphabet: `"cyrillic"` (default), `"latin"`, `"latinJi"` or `"arabic"`.

***

### doEscapeCapitalized

> **doEscapeCapitalized**: `boolean` = `true`

Defined in: [config.ts:37](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L37)

Whether capitalized words are protected from changes (default `true`).

***

### g

> **g**: `boolean` = `true`

Defined in: [config.ts:44](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L44)

Whether to convert `ґ→г`-style `г` into `ґ` where appropriate
(default `true`; `false` in `htmlConfigOptions()`).

***

### j

> **j**: [`TaraskJ`](/taraskevizer/reference/type-aliases/taraskj/) = `'never'`

Defined in: [config.ts:35](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L35)

When to replace `і` by `й` after vowels: `"never"` (default),
`"random"` or `"always"`.

***

### leftAngleBracket

> **leftAngleBracket**: `string` = `'<'`

Defined in: [config.ts:50](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L50)

Replacement for `"<"` (default `"<"`, `"&lt"` in HTML mode).

***

### newLine

> **newLine**: `string` = '\n'

Defined in: [config.ts:48](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L48)

Replacement for `"\n"` (default `"\n"`, `"<br>"` in HTML mode).

***

### noFixPlaceholder

> **noFixPlaceholder**: `string` = ' \uE0FE '

Defined in: [config.ts:52](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L52)

Placeholder for `<…>`-protected spans (default `" \uE0FE "`).

***

### variations

> **variations**: [`TaraskVariations`](/taraskevizer/reference/type-aliases/taraskvariations/) = `'all'`

Defined in: [config.ts:46](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L46)

Which word variation to keep: `"all"` (default), `"no"` or `"first"`.

***

### wrappers

> **wrappers**: [`TaraskWrappers`](/taraskevizer/reference/type-aliases/taraskwrappers/) = `'none'`

Defined in: [config.ts:39](https://github.com/GooseOb/taraskevizer/blob/a9f96a8d4a7e190e1113414385a56ad2c02e146b/src/config.ts#L39)

Active wrapper set: `"none"` (default), `"html"` or `"ansi"`.
