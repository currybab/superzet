---
title: Developing superzet
description: Build, run, and release superzet from source.
---

# Developing superzet

See the platform-specific setup docs for build prerequisites:

- [macOS](./development/macos.md)
- [Linux](./development/linux.md)
- [Windows](./development/windows.md)

## Core Commands

Run the app:

```sh
cargo run -p superzet
```

Check the default lightweight build:

```sh
cargo check -p superzet
```

Run clippy:

```sh
./script/clippy
```

Build the heavier upstream-like surface only when you actually need it:

```sh
cargo build -p superzet --features full
```

## Release Flavors

The default app build is `lite`.

That excludes:

- collab
- calls / WebRTC
- ACP and hosted AI surfaces
- edit prediction and related upstream stacks

`--features full` is still available for debugging inherited upstream behavior.

## Keychain Access

Development builds use a less intrusive credential path so you do not get repeated system keychain prompts while iterating locally.

If you need to test real keychain access in a development build:

```sh
ZED_DEVELOPMENT_USE_KEYCHAIN=1 cargo run -p superzet
```

## Performance Measurements

You can still use the inherited frame measurement tooling:

```sh
export ZED_MEASUREMENTS=1
cargo run -p superzet --release
```

Then compare runs with:

```sh
script/histogram version-a version-b
```

## Release and Docs Workflow

- [Releasing](./development/releasing.md)
- [Release Notes](./development/release-notes.md)
- [Debugging Crashes](./development/debugging-crashes.md)
- [Contributing](https://github.com/nerdface-ai/superzet/blob/main/CONTRIBUTING.md)
- [Security Policy](https://github.com/nerdface-ai/superzet/blob/main/SECURITY.md)
