---
editUrl: false
next: true
prev: true
title: "htmlConfigOptions"
---

> `const` **htmlConfigOptions**: `object`

Defined in: [config.ts:116](https://github.com/GooseOb/taraskevizer/blob/37165f10cf371bf2f1bc325191e2b2edacbf9dbd/src/config.ts#L116)

Predefined configuration for HTML.

## Type Declaration

### g

> `readonly` **g**: `false` = `false`

### leftAngleBracket

> `readonly` **leftAngleBracket**: `"&lt"` = `'&lt'`

### newLine

> `readonly` **newLine**: `"<br>"` = `'<br>'`

### wrappers

> `readonly` **wrappers**: [`Wrappers`](/taraskevizer/reference/taraskevizer/namespaces/wrappers/type-aliases/wrappers/) = `html`

## Example

```ts
const htmlCfg = new TaraskConfig({
  ...myOptions,
  ...htmlConfigOptions
});
```
