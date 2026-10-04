If you're looking for JSON dictionaries, check out the github releases.

[Go to full API documentation](https://gooseob.github.io/taraskevizer/)

# Using \<script> tag

> [!WARNING]
> In production, replace `latest` with a specific version number to avoid breaking changes due to major updates.
> Using `latest` could potentially break your website if a new version introduces breaking changes.

```html
<head>
  <script src="https://cdn.jsdelivr.net/npm/taraskevizer@latest/dist/bundle.js"></script>
  <script>
    // Synchronous, no initialization needed
    document.write(taraskevizer.tarask("планета")); // "плянэта"
  </script>
</head>
```

# Install

With npm:

```sh
npm install taraskevizer
```

With yarn:

```sh
yarn add taraskevizer
```

With bun:

```sh
bun add taraskevizer
```

# Usage

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

const htmlCfg = htmlConfigOptions(); // wrappers: "html", g: false, `<br>` newlines
htmlCfg.abc = "latin"; // g only matters for the cyrillic alphabet
tarask("энергія планеты", htmlCfg);
// "e<tarF>ne</tarF>r<tarF>g</tarF>ija p<tarF>l</tarF>a<tarF>ne</tarF>ty"

alphabetic("яна і іншыя", new TaraskConfig({ abc: "latinJi" }));
// "jana j jinšyja"

// Plain option objects work too, no `TaraskConfig` needed:
tarask("планета", { abc: "latin" });
// "planeta"
```

# Builtin Pipelines

| Pipeline   | Taraskevization | Phonetization     | Alphabet | Special syntax |
| ---------- | --------------- | ----------------- | -------- | -------------- |
| tarask     | ✅              | ❌                | ✅       | ✅             |
| alphabetic | ❌              | ❌                | ✅       | ✅             |
| phonetic   | ❌              | ✅ (experimental) | ✅       | ✅             |

# HTML tags

## tarF

Difference between the input and the output word.

```html
<tarF>this_part_of_word_is_fixed</tarF>

пл<tarF>я</tarF>н
```

## tarL

A part of a word wrapped in this tag is variable,
variations are mentioned in a `data-l` attribute,
separated with commas.

```html
<tarL data-l='variation2,variation3'>variation1</tarL>

<tarL data-l='Горадня'>Гродна</tarL>
```

## tarH

May be toggled between `г`(`h`) and `ґ`(`g`).
Appears only if alphabet is cyrillic.

```html
<tarH>г</tarH>

<tarH>Г</tarH>валт
```

# Special Syntax

|             | fix          | no fix       | change only alphabet |
| ----------- | ------------ | ------------ | -------------------- |
| brackets    | `<,Планета>` | `<Планета>`  | `<*Планета>`         |
| no brackets | `Планета`    | `<.Планета>` | `<*.Планета>`        |

# CLI

The `tarask` command runs the prebuilt Rust binary for your platform —
the same converter as the JS API, but faster, and able to stream multi-GB
inputs with parallel processing. For the full option list, run `tarask --help`.

## Via npm

Installing the package also installs the `tarask` command. The binary
matching your platform is downloaded automatically
from GitHub releases on install (or, failing that, on first run):

With npm:

```sh
npm install -g taraskevizer
```

With yarn:

```sh
yarn global add taraskevizer
```

With bun:

```sh
bun add -g taraskevizer
```

## Usage

```sh
tarask [options] text
```

### "Without installation"

With npm:

```sh
npx taraskevizer [options] text
```

With bun:

```sh
bunx taraskevizer [options] text
```

> [!NOTE]
> The binary is fetched from the release matching the installed package
> version. If it cannot be downloaded (offline install, `--ignore-scripts`),
> download the asset for your platform manually (see below) and place it
> into `<package>/dist/bin/`. Set `TARASKEVIZER_SKIP_BINARY_DOWNLOAD=1`
> to skip the download.

## Native binaries (no runtime needed)

Every release ships standalone `tarask` executables for these platforms:

| OS      | Architecture            | Asset                                |
| ------- | ----------------------- | ------------------------------------ |
| Linux   | x64                     | `tarask-x86_64-unknown-linux-gnu`    |
| Linux   | ARM64                   | `tarask-aarch64-unknown-linux-gnu`   |
| macOS   | x64 (Intel)             | `tarask-x86_64-apple-darwin`         |
| macOS   | ARM64 (Apple Silicon)   | `tarask-aarch64-apple-darwin`        |
| Windows | x64                     | `tarask-x86_64-pc-windows-msvc.exe`  |

Download the file from [GitHub releases](https://github.com/GooseOb/taraskevizer/releases),
make it executable (not needed on Windows) and run it:

```sh
chmod +x tarask-x86_64-unknown-linux-gnu
./tarask-x86_64-unknown-linux-gnu --latin 'планета'
```

# Known bugs

## Replacing `не` with `ня`

`Ня` should appear before a word where the first syllable is stressed.
At the moment, there is no way to check exactly if it is stressed.
Algorithm makes some heuristics, but that's not enough to cover all cases.

# Dependency graph of the project files

![Dependency graph](https://gooseob.github.io/taraskevizer/graph.png)
