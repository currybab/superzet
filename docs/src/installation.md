---
title: Install superzet
description: Install the current public preview of superzet.
---

# Installing superzet

## Public Preview

The current public release target is macOS on Apple Silicon.

Download the latest preview DMG from GitHub Releases:

- [nerdface-ai/superzet releases](https://github.com/nerdface-ai/superzet/releases)

Install it by:

1. downloading `superzet-aarch64.dmg`
2. opening the DMG
3. dragging `superzet` into `/Applications`

After the first bundled install, preview builds can update in-app through the `superzet.dev/releases` update feed.

## Build From Source

For development builds or unsupported public release targets:

```sh
git clone git@github.com:nerdface-ai/superzet.git
cd superzet
cargo run -p superzet
```

Default source builds use the lightweight local shell surface:

```sh
cargo build -p superzet
```

To opt back into the heavier upstream-like surface:

```sh
cargo build -p superzet --features full
```

## Signed macOS Bundles

To build a macOS bundle locally:

```sh
./script/bundle-mac aarch64-apple-darwin
```

For a signed and notarized bundle, the release environment must provide the Apple signing and notarization variables documented in [Releasing](./development/releasing.md).

## Current Platform Scope

- macOS Apple Silicon: public preview release
- macOS Intel / Linux / Windows: source builds and inherited upstream development paths only
