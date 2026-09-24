---
editUrl: false
next: true
prev: true
title: "Pipeline"
---

> **Pipeline**\<`Step`\> = `string`

Defined in: [lib/pipe.ts:5](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/lib/pipe.ts#L5)

## Type Parameters

### Step

`Step` *extends* [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<`any`\> = [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<`any`\>

> **Pipeline**(`text`, `cfg?`): `string`

## Parameters

### text

`string`

### cfg?

#### abc?

\{ `lower`: [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`LowerPattern`\>; `upper?`: [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`UpperPattern`\>; \} = `...`

Predefined alphabets are in [dicts.alphabets](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/readme/).

**Default**

```ts
alphabets.cyrillic
```

#### abc.lower

[`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`LowerPattern`\>

#### abc.upper?

[`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`UpperPattern`\>

#### doEscapeCapitalized?

`boolean` = `...`

If set to false, may cause unwanted changes in acronyms.

**Default**

```ts
true
```

#### g?

`boolean` = `...`

Do replace ґ(g) by г(h) in cyrillic alphabet?

| Value | Example                                 |
| ----- | --------------------------------------- |
| true  | Ґвалт ґвалт                             |
| false | Гвалт гвалт                             |
| true  | `<tarH>ґ</tarH>валт <tarH>Ґ</tarH>валт` |
| false | `<tarH>г</tarH>валт <tarH>Г</tarH>валт` |

**Default**

```ts
false
```

#### j?

[`OptionJ`](/taraskevizer/reference/type-aliases/optionj/) = `...`

| When to replace `і`(`i`) by `й`(`j`) after vowels | Example                  |
| ------------------------------------------------- | ------------------------ |
|                                                   | `яна і ён`               |
| never                                             | `яна і ён`               |
| random                                            | `яна і ён` or `яна й ён` |
| always                                            | `яна й ён`               |

Has no effect with abc set to [dicts.alphabets.latinJi](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/variables/latinji/).

**Default**

```ts
"never"
```

#### leftAngleBracket?

`string` = `...`

String to replace `"<"` with.

**Default**

```ts
"<"
```

**Example**

```ts
"&lt;"
```

#### newLine?

`string` = `...`

String to replace `"\n"` with.

**Default**

```ts
"\n"
```

**Example**

```ts
"<br>"
```

#### noFixPlaceholder?

`string` = `...`

Placeholder for parts that should not be fixed like those enclosed in `< >`.

> Change only if the default value interferes with your text.

**Default**

```ts
" \ue0fe "
```

#### variations?

[`Variation`](/taraskevizer/reference/type-aliases/variation/) = `...`

| Which variation is used if a part of word is variable | Example           |
| ----------------------------------------------------- | ----------------- |
|                                                       | Гродна            |
| no (main)                                             | Гродна            |
| first                                                 | Горадня           |
| all                                                   | (Гродна\|Горадня) |

**Default**

```ts
"all"
```

#### wrappers?

\{ `fix?`: `TransformString`; `letterH?`: `TransformString`; `variable?`: [`VariationWrappers`](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/variationwrappers/); \} \| `null` = `...`

Used for wrapping changed parts.
Predefined dicts are in [wrappers](/taraskevizer/reference/classes/taraskconfig/#wrappers).

If `null`, wrapping changes will be skipped.

**Default**

```ts
null
```

## Returns

`string`

## Properties

### steps

> **steps**: `Step`[]

Defined in: [lib/pipe.ts:7](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/lib/pipe.ts#L7)
