---
editUrl: false
next: true
prev: true
title: "htmlConfigOptions"
---

> `const` **htmlConfigOptions**: `object`

Defined in: [config.ts:116](https://github.com/GooseOb/taraskevizer/blob/180fb29c64d2ce101010ca10e3c9b2ffec5369c0/src/config.ts#L116)

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
