---
editUrl: false
next: true
prev: true
title: "createInteractiveTags"
---

> **createInteractiveTags**(`__namedParameters?`): `object`

Defined in: [html-tag-interactions.ts:13](https://github.com/GooseOb/taraskevizer/blob/180fb29c64d2ce101010ca10e3c9b2ffec5369c0/src/html-tag-interactions.ts#L13)

## Parameters

### \_\_namedParameters?

`Partial`\<`Record`\<`Exclude`\<keyof [`Wrappers`](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/wrappers/), `"fix"`\>, `string`\> & `object`\> = `{}`

## Returns

`object`

### changeList

> **changeList**: `number`[]

### subscribe

> **subscribe**: (`cb`) => () => `void`

#### Parameters

##### cb

`Subscriber`

#### Returns

() => `void`

### tryAlternate

> **tryAlternate**: (`el`) => `void`

#### Parameters

##### el

`ChangeableElement` \| `Element`

#### Returns

`void`

### update

> **update**: (`root`) => `void`

#### Parameters

##### root

`Element`

#### Returns

`void`
