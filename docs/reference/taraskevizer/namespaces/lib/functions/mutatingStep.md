---
editUrl: false
next: true
prev: true
title: "mutatingStep"
---

> **mutatingStep**\<`TStorage`\>(`callback`): [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<`TStorage`\>

Defined in: [lib/mutating-step.ts:23](https://github.com/GooseOb/taraskevizer/blob/180fb29c64d2ce101010ca10e3c9b2ffec5369c0/src/lib/mutating-step.ts#L23)

Utility function for a step that always modifies the text

> Not recommended to use if
a step doesn't ALWAYS modify the text.

## Type Parameters

### TStorage

`TStorage` *extends* `object` = `object`

## Parameters

### callback

(...`args`) => `string`

## Returns

[`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<`TStorage`\>

## Example

```ts
type TextWrapperStorage = { wrapText: (text: string) => string };

const trimStep = mutatingStep<TextWrapperStorage>(
  ({ text, storage: { wrapText } }) => wrapText(text.trim())
);
// is equivalent to
const trimStep: TaraskStep<TextWrapperStorage> = (ctx) => {
  ctx.text = options.storage.wrapText(
    ctx.text.trim()
  );
};
```
