---
title: Releasing superzet
description: Preview release, update feed, and operator setup for superzet.
---

# Releasing superzet

## Public Release Model

The current public release flow is:

- channel: `preview`
- platform: macOS Apple Silicon
- asset host: GitHub Releases
- update feed: `superzet.dev/releases/...` via Cloudflare worker

## Tagging

Publish a preview release by pushing a tag shaped like:

```sh
git tag v0.1.0-pre
git push origin v0.1.0-pre
```

That triggers the release workflow, which:

1. builds a preview bundle with `SUPERZET_RELEASE_CHANNEL=preview`
2. notarizes `superzet-aarch64.dmg`
3. uploads the DMG plus `sha256sums.txt` to the GitHub Release for that tag

## Required GitHub Configuration

Repository secrets:

- `MACOS_CERTIFICATE`
- `MACOS_CERTIFICATE_PASSWORD`
- `APPLE_NOTARIZATION_KEY`
- `APPLE_NOTARIZATION_KEY_ID`
- `APPLE_NOTARIZATION_ISSUER_ID`
- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

Repository variables:

- `MACOS_SIGNING_IDENTITY`

Optional Cloudflare worker secret:

- `GITHUB_RELEASES_TOKEN`

## Required Cloudflare Setup

Deploy the workers in `.cloudflare/` so these routes exist:

- `superzet.dev/docs*`
- `superzet.dev/releases*`

The release worker serves two behaviors:

- `/releases/{channel}/{version}/asset?...` returns JSON for the app updater
- `/releases/{channel}/{version}` redirects to the matching GitHub Release page

If you expect more than light tester traffic, set `GITHUB_RELEASES_TOKEN` on the worker so GitHub API lookups are not limited to anonymous request quotas.

## Update Feed Contract

The updater expects:

```json
{
  "version": "0.1.0",
  "url": "https://github.com/nerdface-ai/superzet/releases/download/..."
}
```

For preview releases, Git tags still use `-pre`, but the update worker strips that suffix before returning `version` so app-side semver comparison continues to work.

## Current Non-Goals

These are not part of the first public release flow:

- Linux or Windows binary publishing
- remote-server asset publishing
- collab deployment
- hosted AI deployment
- Sentry or other upstream release automation
