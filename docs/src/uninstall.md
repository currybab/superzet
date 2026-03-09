---
title: Uninstall
description: Remove superzet from your machine.
---

# Uninstall

## macOS

If you installed the preview DMG build:

1. quit `superzet`
2. drag `/Applications/superzet.app` to the Trash
3. empty the Trash if you want the bundle removed immediately

## Optional: Remove Local Data

To remove local app data as well, delete these paths if they exist:

- `~/Library/Application Support/superzet`
- `~/Library/Caches/ai.nerdface.superzet`
- `~/Library/Logs/superzet`
- `~/Library/Saved Application State/ai.nerdface.superzet.savedState`

If you are using preview, nightly, or dev channels, remove the matching bundle identifier paths for those channels instead.

## Source Builds

If you only ran `superzet` from source, removing the checkout and its build output is enough:

```sh
rm -rf target
```

Remove local app data separately if you want a clean reset.
