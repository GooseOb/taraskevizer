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
  pipelines,
  TaraskConfig,
  htmlConfigOptions,
  wrappers,
  dicts,
} from "taraskevizer";

pipelines.tarask("планета");
// "плянэта"

const cfg = new TaraskConfig({
  abc: dicts.alphabets.cyrillic,
  j: "always",
  variations: "first",
  wrappers: wrappers.ansiColor,
  g: true,
});
pipelines.tarask("планета і Гродна", cfg);
// "пл\x1b[32mя\x1b[0mн\x1b[32mэ\x1b[0mта \x1b[32mй\x1b[0m \x1b[35mГорадня\x1b[0m"

pipelines.tarask("энергія планеты", {
  ...htmlConfigOptions,
  abc: dicts.alphabets.latin,
  g: false, // ignored, g matters for cyrillic alphabet only
});
// "en<tarF>erg</tarF>ija p<tarF>lan</tarF>ety"

const latinWithJiCfg = new TaraskConfig({
  abc: dicts.alphabets.latinJi,
});

pipelines.alphabetic("яна і іншыя", latinWithJiCfg);
// "jana j jinšyja"
```

See the [API reference](/taraskevizer/reference/readme/) for every export, option, and type.

## The two moving parts: pipelines and config

Every conversion is a call to a **pipeline** (what to do) with an optional
[**`TaraskConfig`**](/taraskevizer/reference/classes/taraskconfig/) (how to do it).

- A **pipeline** is a preconfigured routine such as `pipelines.tarask`,
  `pipelines.alphabetic`, or `pipelines.phonetic`. See
  [Builtin pipelines](/taraskevizer/guides/pipelines/).
- A **config** tunes the alphabet, letter replacements, variations, and how
  changed parts are wrapped. See
  [Configuration](/taraskevizer/guides/configuration/).

If you omit the config, the default `TaraskConfig` is used, which converts
into the Belarusian classical orthography (тарашкевіца) using the Cyrillic
alphabet.

## Using a `<script>` tag

The package ships a browser bundle. After loading it, everything is exposed
on the global `taraskevizer` object.

:::caution
In production, replace `latest` with a specific version number to avoid
breaking changes due to major updates. Using `latest` could break your
website if a new version introduces breaking changes.
:::

```html
<head>
  <script src="https://cdn.jsdelivr.net/npm/taraskevizer@latest/dist/bundle.js"></script>
  <script>
    document.write(taraskevizer.pipelines.tarask("планета"));
  </script>
</head>
```

The global mirrors the module exports, so `taraskevizer.pipelines.tarask(...)`
works exactly like the imported `pipelines.tarask(...)`.
