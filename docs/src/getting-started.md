---
title: Getting Started with superzet
description: Get started with superzet, the local-first workspace shell for coding agents.
---

# Getting Started

`superzet` is a local-first shell for working across repositories, worktrees, terminals, diffs, and editor panes in one native app window.

## Quick Start

### 1. Open the app and add a repository

The main window is built around multiple local workspaces. Use the welcome page or sidebar controls to open an existing repository, then create or switch worktrees from there.

### 2. Learn the core shortcuts

| Action                      | macOS          |
| --------------------------- | -------------- |
| Command palette             | `Cmd+Shift+P`  |
| Open file                   | `Cmd+P`        |
| Search symbols              | `Ctrl+Shift+T` |
| New terminal in center pane | `Cmd+T`        |
| Toggle terminal panel       | `` Ctrl+` ``   |
| Open settings               | `Cmd+,`        |

### 3. Work from terminals first

The default `superzet` workflow expects you to run external coding agents and local tooling from terminals.

- Use `Cmd+T` for a center-pane terminal tab
- Use `` Ctrl+` `` to show or hide the terminal panel
- Keep diffs, files, and terminals visible at the same time instead of constantly context switching

### 4. Tune the workspace shell

Good first settings:

- theme and font
- startup workspace behavior
- keybindings
- project-specific tasks

See [Running & Testing](./running-testing.md), [Terminal](./terminal.md), and [All Settings](./reference/all-settings.md).

## Current Product Scope

The default build is intentionally narrow:

- local repositories and worktrees
- native editor, panes, and diffs
- terminal-driven agent workflows

Not part of the public preview scope:

- collaboration
- hosted AI surfaces
- remote-server distribution

## Public Preview Availability

The first public binary release is currently macOS preview only. Other platforms are still source-build territory.
