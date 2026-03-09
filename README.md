# superzet

<p align="center">
  <img src="assets/branding/logo_default.png" alt="superzet" width="180" />
</p>

`superzet` is a local-first workspace shell for coding agents.

It is built on top of a Zed fork, but the product scope is narrower: one window, multiple local workspaces, fast file navigation, diff views, and terminal-heavy agent workflows.

## Status

This repository is in early alpha.

Current focus:

- local repositories and git worktrees
- native editor, split panes, and diff views
- terminal-first use of external coding agents
- macOS public preview releases

Deliberately out of scope for the default build:

- cloud collaboration
- calls / WebRTC
- hosted AI surfaces from upstream Zed
- remote-server distribution as part of the public release flow

## Build From Source

```bash
git clone git@github.com:nerdface-ai/superzet.git
cd superzet
cargo run -p superzet
```

The default app build is the lightweight single-user shell flavor:

```bash
cargo build -p superzet
```

If you need the upstream-like AI and collab stack again:

```bash
cargo build -p superzet --features full
```

For a signed macOS bundle:

```bash
./script/bundle-mac aarch64-apple-darwin
```

## Public Preview Release

The public test release flow is macOS preview only.

- Tag preview releases as `vX.Y.Z-pre`
- GitHub Actions builds `superzet-aarch64.dmg`
- The workflow uploads the DMG and `sha256sums.txt` to GitHub Releases
- `superzet.dev/releases/...` is served by a thin Cloudflare worker that points the app at those GitHub assets

## Release Infrastructure

To publish preview builds with in-app updates, you need:

- GitHub Releases enabled for this repository
- Cloudflare configured for `superzet.dev`
- the release worker deployed from `.cloudflare/release-assets`
- optional but recommended: a Cloudflare worker secret named `GITHUB_RELEASES_TOKEN` to avoid GitHub API rate limits on update checks
- Apple signing and notarization credentials in GitHub secrets:
  - `MACOS_CERTIFICATE`
  - `MACOS_CERTIFICATE_PASSWORD`
  - `APPLE_NOTARIZATION_KEY`
  - `APPLE_NOTARIZATION_KEY_ID`
  - `APPLE_NOTARIZATION_ISSUER_ID`
- the mac signing identity in the `MACOS_SIGNING_IDENTITY` repository variable

The app prefers `SUPERZET_*` runtime env vars for release/update overrides, but legacy `ZED_*` aliases still work during the transition.

## Open Source Notes

- Extensions still use the upstream Zed marketplace.
- Much of the editor and platform code still comes from upstream Zed and is intentionally kept close for easier maintenance.
- Public docs and release surfaces are being rewritten around `superzet`, but not every inherited upstream page has been reworked yet.

## Project Docs

- [Contributing](./CONTRIBUTING.md)
- [Security](./SECURITY.md)
- [Docs](./docs/src/SUMMARY.md)

## License

This repository remains GPL-3.0-or-later, consistent with the current fork base.
