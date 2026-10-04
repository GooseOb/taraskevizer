---
editUrl: false
next: true
prev: true
title: "createInteractiveTags"
---

> **createInteractiveTags**(`__namedParameters?`): `object`

Defined in: [html-tag-interactions.ts:26](https://github.com/GooseOb/taraskevizer/blob/ac801db8bb7595f43c3cc25af94c04cfdeaad4c1/src/html-tag-interactions.ts#L26)

## Parameters

### \_\_namedParameters?

`Partial`\<`Record`\<`"variable"` \| `"letterH"`, `string`\> & `object`\> = `{}`

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
