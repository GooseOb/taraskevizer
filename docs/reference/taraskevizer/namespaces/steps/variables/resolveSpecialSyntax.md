---
editUrl: false
next: true
prev: true
title: "resolveSpecialSyntax"
---

> `const` **resolveSpecialSyntax**: [`TaraskStep`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/taraskstep/)\<[`SpecialSyntaxStorage`](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/specialsyntaxstorage/)\>

Defined in: [steps/resolve-syntax.ts:38](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/steps/resolve-syntax.ts#L38)

Captures noFix parts and stores them in [SpecialSyntaxStorage.noFixArr](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/specialsyntaxstorage/#nofixarr).
Places a [TaraskConfig.noFixPlaceholder](/taraskevizer/reference/classes/taraskconfig/#nofixplaceholder) in their place.

Use [applyNoFix](/taraskevizer/reference/taraskevizer/namespaces/steps/variables/applynofix/) to bring the parts back to the text.

Creates storage: [SpecialSyntaxStorage](/taraskevizer/reference/taraskevizer/namespaces/steps/type-aliases/specialsyntaxstorage/).
