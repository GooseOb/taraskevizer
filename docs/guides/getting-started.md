---
title: Getting started
description: Install taraskevizer and run your first conversion.
sidebar:
  order: 1
---

## Install

import into a project with your package manager of choice:

```sh
npm install taraskevizer
# or
yarn add taraskevizer
# or
bun add taraskevizer
```

## Usage

```js
import {
  init,
  tarask,
  alphabetic,
  TaraskConfig,
  htmlConfigOptions,
} from "taraskevizer";

await init(); // skip under Node.js: the `node` export initializes automatically

tarask("планета");
// "плянэта"

const cfg = new TaraskConfig({
  abc: "cyrillic",
  j: "always",
  variations: "first",
  wrappers: "ansi",
  g: true,
});
tarask("планета і Гродна", cfg);
// "пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m"

const htmlCfg = htmlConfigOptions();
htmlCfg.abc = "latin";
tarask("энергія планеты", htmlCfg);
// "e<tarF>ne</tarF>r<tarF>g</tarF>ija p<tarF>l</tarF>a<tarF>ne</tarF>ty"

alphabetic("яна і іншыя", new TaraskConfig({ abc: "latinJi" }));
// "jana j jinšyja"
```

See the [API reference](/taraskevizer/reference/readme/) for every export, option, and type.

## The two moving parts: pipelines and config

Every conversion is a call to a **pipeline** (what to do) with an optional
[**`TaraskConfig`**](/taraskevizer/reference/classes/taraskconfig/) (how to do it).

- A **pipeline** is an exported conversion function such as
  [`tarask`](/taraskevizer/reference/functions/tarask/),
  [`alphabetic`](/taraskevizer/reference/functions/alphabetic/), or
  [`phonetic`](/taraskevizer/reference/functions/phonetic/). See
  [Builtin pipelines](/taraskevizer/guides/pipelines/).
- A **config** tunes the alphabet, letter replacements, variations, and how
  changed parts are wrapped. It can be a `TaraskConfig` instance or a plain
  option object (`tarask("планета", { abc: "latin" })`). See
  [Configuration](/taraskevizer/guides/configuration/).

If you omit the config, the defaults are used, which convert into the
Belarusian classical orthography (тарашкевіца) using the Cyrillic alphabet.

## Using a `<script>` tag

The package ships a browser bundle. After loading it, everything is exposed
on the global `taraskevizer` object. The bundle is fully synchronous — no
initialization call is needed.

:::caution
In production, replace `latest` with a specific version number to avoid
breaking changes due to major updates. Using `latest` could break your
website if a new version introduces breaking changes.
:::

```html
<head>
  <script src="https://cdn.jsdelivr.net/npm/taraskevizer@latest/dist/bundle.js"></script>
  <script>
    document.write(taraskevizer.tarask("планета")); // "плянэта"
  </script>
</head>
```

The global mirrors the module exports, so `taraskevizer.tarask(...)`
works exactly like the imported `tarask(...)`.
