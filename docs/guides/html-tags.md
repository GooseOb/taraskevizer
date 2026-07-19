---
title: HTML tags
description: Custom HTML tags emitted by the HTML wrappers and how to make them interactive.
sidebar:
  order: 4
---

When converting with the HTML wrappers (`wrappers.html`, or
`htmlConfigOptions`), the output is annotated with custom tags that describe
what happened to each word. This is useful for building interactive
previews where readers can toggle between spelling or alphabet variants.

## `tarF`

Marks the difference between the input and the output word — the part that was
actually changed.

```html
<tarF>this_part_of_word_is_fixed</tarF>

пл<tarF>я</tarF>н
```

## `tarL`

Wraps a part of a word that is **variable** — a word with more than one valid
spelling. The alternatives are listed in a `data-l` attribute, separated by
commas. Which one is shown as the element body is controlled by the
`variations` config option.

```html
<tarL data-l="variation2,variation3">variation1</tarL>

<tarL data-l="Горадня">Гродна</tarL>
```

## `tarH`

Wraps the letter ґ(`g`)/г(`h`). It can be toggled between the two forms and
appears only when the alphabet is Cyrillic (i.e. `g` matters).

```html
<tarH>г</tarH>

<tarH>Г</tarH>валт
```

## Making the tags interactive

[`createInteractiveTags`](/taraskevizer/reference/functions/createinteractivetags/)
returns a controller you can use to flip between variants in the DOM at
runtime. Call `update(root)` once to register all `tarL`/`tarH` elements, then
let the user toggle them.

```js
// This example runs in the browser: it uses the DOM (`document`).
import { pipelines, htmlConfigOptions, createInteractiveTags } from "taraskevizer";

const html = pipelines.tarask("Гродна і ґвалт", { ...htmlConfigOptions });
document.body.innerHTML = html;

const tags = createInteractiveTags();
// register every tarL / tarH element under <body>
tags.update(document.body);

// later, toggle the variant of a clicked element
document.body.addEventListener("click", (e) => {
  tags.tryAlternate(e.target);
});
```

### Controller API

The object returned by `createInteractiveTags(options?)` has:

- **`update(root)`** — scans `root` for `tarL`/`tarH` elements and (re)builds
  the internal `changeList`. Call it again whenever the DOM changes.
- **`tryAlternate(el)`** — advances `el` to its next variant (next `data-l`
  alternative, or toggles ґ/г for `tarH`).
- **`subscribe(cb)`** — registers a callback `(changeList) => void` that fires
  whenever the selection changes. Returns an unsubscribe function.
- **`changeList`** — an array of indices, one per registered element, tracking
  which variant is currently selected.

You can pass options to `createInteractiveTags` to rename the tags it looks
for (`variable` and `letterH`, both defaulting to `"tarL"`/`"tarH"`) and to
seed an initial `changeList`.
