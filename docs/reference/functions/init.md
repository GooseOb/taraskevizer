---
editUrl: false
next: true
prev: true
title: "init"
---

> **init**(`module_or_path?`): `Promise`\<`InitOutput`\>

Defined in: wasm/taraskevizer\_wasm.d.ts:125

If `module_or_path` is {RequestInfo} or {URL}, makes a request and
for everything else, calls `WebAssembly.instantiate` directly.

## Parameters

### module\_or\_path?

\{ `module_or_path`: InitInput \| Promise\<InitInput\>; \} \| `InitInput` \| `Promise`\<`InitInput`\>

Passing `InitInput` directly is deprecated.

## Returns

`Promise`\<`InitOutput`\>
