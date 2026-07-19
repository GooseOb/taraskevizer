---
editUrl: false
next: true
prev: true
title: "Alphabet"
---

> **Alphabet**\<`LowerPattern`, `UpperPattern`\> = `object`

Defined in: [dict/alphabets/types.ts:9](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/dict/alphabets/types.ts#L9)

If [Alphabet.upper](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/type-aliases/alphabet/#upper) in not defined,
it is assumed that the alphabet is case-insensitive,
so [Alphabet.lower](/taraskevizer/reference/taraskevizer/namespaces/dicts/namespaces/alphabets/type-aliases/alphabet/#lower) should replace both
upper and lower case letters.

## Type Parameters

### LowerPattern

`LowerPattern` = `RegExp`

### UpperPattern

`UpperPattern` = `LowerPattern`

## Properties

### lower

> **lower**: [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`LowerPattern`\>

Defined in: [dict/alphabets/types.ts:10](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/dict/alphabets/types.ts#L10)

***

### upper?

> `optional` **upper?**: [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)\<`UpperPattern`\>

Defined in: [dict/alphabets/types.ts:11](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/dict/alphabets/types.ts#L11)
