## Install

```
$ npm i taraskevizer
```


## Usage

```js
import { taraskSync, tarask } from 'taraskevizer';

const result = taraskSync(text);
// or
const result = await tarask(text);
```

## API

### tarask(text, options?)
Returns a `Promise<string>`

### taraskSync(text, options?)
Returns a `string`

#### text
Type: `string`

## Options
Type: `object`

### abc
Type: `number`

Default value: `0`

Alphabet of output text:
0. cyrillic
1. latin
2. arabic

### j
Type: `number`

Default value: `0`

When to replace `і`(`i`) by `й`(`j`) after vowels:
0. never
1. random
2. always

### html
Type: `false|object`

Default value: `false`

If `object`, some parts of a text are wrapped in HTML tags.

#### g
Type: `boolean`

Default value: `false`

Do replace `г`(`h`) by `ґ`(`g`) in cyrillic alphabet?
```html
false: <tarH>г</tarH> <tarH>Г</tarH>
true:  <tarH>ґ</tarH> <tarH>Ґ</tarH>
```

## HTML tags

### tarF
Difference between an input and an output word.

```html
<tarF>this_part_of_word_is_fixed</tarF>

пл<tarF>я</tarF>н
```

### tarL
A part of a word wrapped in this tag is variable,
variations are mentioned in a `data-l` attribute
and are separated by commas

```html
<tarL data-l="variation2,variation3">variation1</tarL>

<tarL data-l="Горадня">Гродна</tarL>
```

### tarH
Can be replaced by `ґ`(`g`) letter. appears only if alphabet is cyrillic

```html
<tarH>г</tarH>

<tarH>Г</tarH>валт
```
