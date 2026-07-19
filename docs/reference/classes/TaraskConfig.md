---
editUrl: false
next: true
prev: true
title: "TaraskConfig"
---

Defined in: [config.ts:9](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L9)

## Constructors

### Constructor

> **new TaraskConfig**(`options?`): `TaraskConfig`

Defined in: [config.ts:10](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L10)

#### Parameters

##### options?

`Partial`\<\{ `abc`: \{ `lower`: [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`RegExp`\>; `upper?`: [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`RegExp`\>; \}; `doEscapeCapitalized`: `boolean`; `g`: `boolean`; `j`: [`OptionJ`](/taraskevizer/reference/type-aliases/optionj/); `leftAngleBracket`: `string`; `newLine`: `string`; `noFixPlaceholder`: `string`; `variations`: [`Variation`](/taraskevizer/reference/type-aliases/variation/); `wrappers`: \{ `fix?`: `TransformString`; `letterH?`: `TransformString`; `variable?`: \{ `all`: `TransformString`; `first`: `TransformString`; `no`: `TransformString`; \}; \} \| `null`; \}\>

#### Returns

`TaraskConfig`

## Properties

### abc

> **abc**: [`Alphabet`](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/type-aliases/alphabet/)

Defined in: [config.ts:22](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L22)

Predefined alphabets are in [dicts.alphabets](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/readme/).

#### Default

```ts
alphabets.cyrillic
```

***

### doEscapeCapitalized

> **doEscapeCapitalized**: `boolean`

Defined in: [config.ts:43](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L43)

If set to false, may cause unwanted changes in acronyms.

#### Default

```ts
true
```

***

### g

> **g**: `boolean`

Defined in: [config.ts:67](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L67)

Do replace ґ(g) by г(h) in cyrillic alphabet?

| Value | Example                                 |
| ----- | --------------------------------------- |
| true  | Ґвалт ґвалт                             |
| false | Гвалт гвалт                             |
| true  | `<tarH>ґ</tarH>валт <tarH>Ґ</tarH>валт` |
| false | `<tarH>г</tarH>валт <tarH>Г</tarH>валт` |

#### Default

```ts
false
```

***

### j

> **j**: [`OptionJ`](/taraskevizer/reference/type-aliases/optionj/)

Defined in: [config.ts:36](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L36)

| When to replace `і`(`i`) by `й`(`j`) after vowels | Example                  |
| ------------------------------------------------- | ------------------------ |
|                                                   | `яна і ён`               |
| never                                             | `яна і ён`               |
| random                                            | `яна і ён` or `яна й ён` |
| always                                            | `яна й ён`               |

Has no effect with abc set to [dicts.alphabets.latinJi](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/variables/latinji/).

#### Default

```ts
"never"
```

***

### leftAngleBracket

> **leftAngleBracket**: `string`

Defined in: [config.ts:95](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L95)

String to replace `"<"` with.

#### Default

```ts
"<"
```

#### Example

```ts
"&lt;"
```

***

### newLine

> **newLine**: `string`

Defined in: [config.ts:87](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L87)

String to replace `"\n"` with.

#### Default

```ts
"\n"
```

#### Example

```ts
"<br>"
```

***

### noFixPlaceholder

> **noFixPlaceholder**: `string`

Defined in: [config.ts:104](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L104)

Placeholder for parts that should not be fixed like those enclosed in `< >`.

> Change only if the default value interferes with your text.

#### Default

```ts
" \ue0fe "
```

***

### variations

> **variations**: [`Variation`](/taraskevizer/reference/type-aliases/variation/)

Defined in: [config.ts:79](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L79)

| Which variation is used if a part of word is variable | Example           |
| ----------------------------------------------------- | ----------------- |
|                                                       | Гродна            |
| no (main)                                             | Гродна            |
| first                                                 | Горадня           |
| all                                                   | (Гродна\|Горадня) |

#### Default

```ts
"all"
```

***

### wrappers

> **wrappers**: `Partial`\<[`Wrappers`](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/wrappers/)\> \| `null`

Defined in: [config.ts:53](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L53)

Used for wrapping changed parts.
Predefined dicts are in [wrappers](/taraskevizer/reference/classes/taraskconfig/#wrappers).

If `null`, wrapping changes will be skipped.

#### Default

```ts
null
```
