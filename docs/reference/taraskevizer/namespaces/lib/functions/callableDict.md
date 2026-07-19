---
editUrl: false
next: true
prev: true
title: "callableDict"
---

> **callableDict**(`value`): [`CallableDict`](/taraskevizer/reference/taraskevizer/namespaces/dicts/type-aliases/callabledict/)

Defined in: [dict/lib.ts:15](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/dict/lib.ts#L15)

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
