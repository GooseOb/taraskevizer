---
editUrl: false
next: true
prev: true
title: "_pipe"
---

> **\_pipe**(`subPipeline`): [`Pipeline`](/taraskevizer/reference/taraskevizer/namespaces/lib/type-aliases/pipeline/)\<[`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`SplittedTextStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/splittedtextstorage/)\> \| [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`SpecialSyntaxStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/specialsyntaxstorage/)\> \| [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`WhiteSpaceStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/whitespacestorage/)\> \| [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`TrimStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/trimstorage/)\>\>

Defined in: [pipelines.ts:77](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/pipelines.ts#L77)

For better tree-shaking instead of `Array.prototype.flatMap`

Used by [tarask](/taraskevizer/reference/taraskevizer/namespaces/pipelines/variables/tarask/) and [phonetic](/taraskevizer/reference/taraskevizer/namespaces/pipelines/variables/phonetic/).

## Parameters

### subPipeline

[`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<`any`\>[]

Steps used instead of [[steps.taraskevize](/taraskevizer/reference/taraskevizer/namespaces/steps/variables/taraskevize/)].

## Returns

[`Pipeline`](/taraskevizer/reference/taraskevizer/namespaces/lib/type-aliases/pipeline/)\<[`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`SplittedTextStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/splittedtextstorage/)\> \| [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`SpecialSyntaxStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/specialsyntaxstorage/)\> \| [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`WhiteSpaceStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/whitespacestorage/)\> \| [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`TrimStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/trimstorage/)\>\>
