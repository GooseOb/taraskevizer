---
editUrl: false
next: true
prev: true
title: "htmlConfigOptions"
---

> `const` **htmlConfigOptions**: `object`

Defined in: [config.ts:116](https://github.com/GooseOb/taraskevizer/blob/04e4f66503c2dd917f7704af253eb5209d50b02d/src/config.ts#L116)

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
