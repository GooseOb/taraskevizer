---
editUrl: false
next: true
prev: true
title: "finalize"
---

> `const` **finalize**: [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<`object`\>

Defined in: [steps/finalize.ts:15](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/steps/finalize.ts#L15)

Reverse the changes made in the [prepare](/taraskevizer/reference/taraskevizer/namespaces/steps/variables/prepare/) step
and replace new lines with the passed string.

Restores:
`(` from `&#40`,
` ` from `&nbsp;`,

Replaces new lines with the `newLine` config value.

Removes spaces around punctuation marks and digits.
