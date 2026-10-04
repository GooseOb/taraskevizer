---
editUrl: false
next: true
prev: true
title: "TaraskConfig"
---

Defined in: [index.ts:55](https://github.com/GooseOb/taraskevizer/blob/f771aee86ae4eba9640f1c12193079b8d1dcaac2/src/index.ts#L55)

Conversion options. Mirrors the previous `TaraskConfig` API: every field
keeps its name (`doEscapeCapitalized`, `newLine`, …) and defaults.
Only predefined wrapper sets are available (`wrappers: 'none' | 'html' |
'ansi'`); arbitrary callback wrappers cannot cross the WASM boundary.

## Extends

- `TaraskConfig`

## Constructors

### Constructor

> **new TaraskConfig**(`options?`): `TaraskConfig`

Defined in: [index.ts:56](https://github.com/GooseOb/taraskevizer/blob/f771aee86ae4eba9640f1c12193079b8d1dcaac2/src/index.ts#L56)

#### Parameters

##### options?

[`TaraskOptions`](/taraskevizer/reference/interfaces/taraskoptions/)

#### Returns

`TaraskConfig`

#### Overrides

`TaraskConfigBase.constructor`

## Properties

### abc

> **abc**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:48

Alphabet: `"cyrillic"` (default), `"latin"`, `"latinJi"` or `"arabic"`.

#### Inherited from

`TaraskConfigBase.abc`

***

### doEscapeCapitalized

> **doEscapeCapitalized**: `boolean`

Defined in: wasm/taraskevizer\_wasm.d.ts:52

Whether capitalized words are protected from changes (default `true`).

#### Inherited from

`TaraskConfigBase.doEscapeCapitalized`

***

### g

> **g**: `boolean`

Defined in: wasm/taraskevizer\_wasm.d.ts:57

Whether to convert `ґ→г`-style `г` into `ґ` where appropriate
(default `true`; `false` in [`html_config_options`]).

#### Inherited from

`TaraskConfigBase.g`

***

### j

> **j**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:62

When to replace `і` by `й` after vowels: `"never"` (default),
`"random"` or `"always"`.

#### Inherited from

`TaraskConfigBase.j`

***

### leftAngleBracket

> **leftAngleBracket**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:66

Replacement for `"<"` (default `"<"`, `"&lt"` in HTML mode).

#### Inherited from

`TaraskConfigBase.leftAngleBracket`

***

### newLine

> **newLine**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:70

Replacement for `"\n"` (default `"\n"`, `"<br>"` in HTML mode).

#### Inherited from

`TaraskConfigBase.newLine`

***

### noFixPlaceholder

> **noFixPlaceholder**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:74

Placeholder for `<…>`-protected spans (default `" \u{e0fe} "`).

#### Inherited from

`TaraskConfigBase.noFixPlaceholder`

***

### variations

> **variations**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:78

Which word variation to keep: `"all"` (default), `"no"` or `"first"`.

#### Inherited from

`TaraskConfigBase.variations`

***

### wrappers

> **wrappers**: `string`

Defined in: wasm/taraskevizer\_wasm.d.ts:82

Active wrapper set: `"none"` (default), `"html"` or `"ansi"`.

#### Inherited from

`TaraskConfigBase.wrappers`

## Methods

### \[dispose\]()

> **\[dispose\]**(): `void`

Defined in: wasm/taraskevizer\_wasm.d.ts:30

#### Returns

`void`

#### Inherited from

`TaraskConfigBase.[dispose]`

***

### free()

> **free**(): `void`

Defined in: wasm/taraskevizer\_wasm.d.ts:29

#### Returns

`void`

#### Inherited from

`TaraskConfigBase.free`
