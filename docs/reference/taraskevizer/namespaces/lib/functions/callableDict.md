---
editUrl: false
next: true
prev: true
title: "callableDict"
---

> **callableDict**(`value`): [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)

Defined in: [dict/lib.ts:15](https://github.com/GooseOb/taraskevizer/blob/180fb29c64d2ce101010ca10e3c9b2ffec5369c0/src/dict/lib.ts#L15)

## Parameters

### value

[`Dict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/dict/)

## Returns

[`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)

function with property `value` that references the dictionary
passed as an argument.

It is possible to change the dictionary after initialization by modifying
the `value` property.

You can use [copyDict](/taraskevizer/reference/taraskevizer/namespaces/lib/functions/copydict/) before passing the dictionary to this function
