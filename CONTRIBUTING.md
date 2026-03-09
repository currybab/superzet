# Contributing to superzet

Thanks for contributing.

`superzet` is currently a local-first shell for managing repositories, worktrees, terminals, and editor state in one native app. Contributions should reinforce that scope rather than reintroduce upstream Zed cloud, collab, or hosted AI assumptions by default.

## Before You Start

- Use [GitHub Discussions](https://github.com/nerdface-ai/superzet/discussions) for feature ideas and design discussion.
- Use [GitHub Issues](https://github.com/nerdface-ai/superzet/issues) for reproducible bugs and concrete implementation work.
- If you plan to work on release flow, docs publishing, or update infrastructure, read [docs/src/development/releasing.md](./docs/src/development/releasing.md) first.

## Local Development

Run the default app:

```bash
cargo run -p superzet
```

Default builds use the lightweight local shell surface:

```bash
cargo build -p superzet
```

If you need the upstream-like AI or collab stack:

```bash
cargo build -p superzet --features full
```

Useful checks:

```bash
cargo check -p superzet
./script/clippy
```

For a macOS preview bundle:

```bash
./script/bundle-mac aarch64-apple-darwin
```

## Change Guidelines

- Prefer changes that improve the local-first workspace shell.
- Do not add new cloud, collab, or hosted AI behavior to the default build without prior discussion.
- Keep user-facing naming, docs, release assets, and links branded as `superzet`.
- If a surface intentionally still points at upstream Zed, call that out explicitly in code review or docs.
- Prefer editing existing files and existing crates over adding new layers.

## Workflows and Generated Files

Some GitHub workflow files are generated from `tooling/xtask`.

If you change workflow behavior:

```bash
cargo run -p xtask -- workflows
```

Do not hand-edit generated workflow YAML unless you are also updating the xtask source.

## Pull Requests

Every PR should include:

- focused scope
- verification notes or screenshots for UI changes
- tests when the change is testable
- a `Release Notes:` section in the PR body

Use one of:

- `- Added ...`
- `- Fixed ...`
- `- Improved ...`
- `- N/A`

`Release Notes:` lines are still required even though the current preview release workflow uses GitHub-generated release notes. We keep the PR notes for human review and future curated release summaries.

## Docs and Community Files

If your change affects installation, updates, release packaging, or public OSS workflows, update the corresponding docs in the same PR:

- [README.md](./README.md)
- [SECURITY.md](./SECURITY.md)
- [docs/src/installation.md](./docs/src/installation.md)
- [docs/src/update.md](./docs/src/update.md)
- [docs/src/development/releasing.md](./docs/src/development/releasing.md)

## Contributor Conduct

This repository follows the rules in [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
