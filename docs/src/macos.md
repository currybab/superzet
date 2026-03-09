---
title: superzet on macOS
description: Install and run the current public preview of superzet on macOS.
---

# superzet on macOS

macOS is the first public release target for `superzet`.

## Installing the Preview Build

Download the latest preview DMG from:

- [GitHub Releases](https://github.com/nerdface-ai/superzet/releases)

Then:

1. open `superzet-aarch64.dmg`
2. drag `superzet.app` into `/Applications`
3. launch the app from Applications

## Building From Source

For local development:

```sh
cargo run -p superzet
```

For a bundled macOS build:

```sh
./script/bundle-mac aarch64-apple-darwin
```

## Current Support Level

- public preview binary: Apple Silicon
- source builds: inherited upstream development paths still exist for Intel and other platforms, but they are not the focus of the public release flow

## Updates

Bundled preview builds can update through the `superzet.dev/releases` feed.

Development builds and source builds should be updated manually.

## Troubleshooting

### Gatekeeper or quarantine warnings

If macOS blocks the app after download, remove the quarantine attribute:

```sh
xattr -cr /Applications/superzet.app
```

### Log file

Use the command palette to open the log, or inspect:

```sh
~/Library/Logs/superzet/superzet.log
```
