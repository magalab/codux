# Codux Apps

This directory contains runnable Codux applications. Shared protocol, runtime, terminal, and transport code lives in `../crates`; app directories should only own product-specific UI, packaging, platform integration, or process entry points.

## Applications

| Path | Role | Runtime |
| --- | --- | --- |
| `desktop/` | Rust + GPUI desktop app. Owns the primary UI, local workspace orchestration, AI CLI sessions, local terminal adapter, and host-side remote runtime. | Rust |
| `mobile/` | Flutter mobile controller. Connects to a Codux host, renders remote runtime state, and sends user intent. | Flutter + Rust FFI |
| `agent/` | Headless controlled-agent entry point. Uses shared protocol, transport, runtime, and PTY crates without GPUI. | Rust |

## Commands

Run from the repository root:

```bash
just desktop
just mobile android
just mobile ios
just agent -- --version
just test
```

Use app-local commands only when working inside that app's native toolchain, such as `flutter test` in `apps/mobile`.

## Release Flow

This repository owns the source and release workflows for the desktop app, mobile app, and headless Agent.

| Product | Release tag | Workflow | Output |
| --- | --- | --- | --- |
| Desktop + Agent | `vX.Y.Z` | `release-gpui.yml`, followed by the formal macOS workflow | Desktop installers, updater metadata, Agent binaries, and (for stable releases) Homebrew cask update |
| Android | `mobile-vX.Y.Z` | `release-mobile-android.yml` | Signed Android APK attached to a GitHub Release |

The release workflows publish to the GitHub repository that runs them. Set the required repository secrets before release: Tauri updater signing keys for updater artifacts; Apple signing/notarization credentials for formally signed macOS packages; Android signing credentials for APK releases. Homebrew publishing also requires a tap token.

Before tagging, update the changelog and mobile version as applicable, validate the target builds, and confirm the workflow run publishes assets to this repository. Desktop versions are applied from the `vX.Y.Z` tag by the release workflow. Android versions must match `apps/mobile/pubspec.yaml` and the `mobile-vX.Y.Z` tag.

## Ownership Rules

- UI code belongs in an app directory.
- Shared protocol names, payload shapes, transport rules, terminal state, and reusable runtime models belong in `../crates`.
- Do not duplicate Iroh transport selection, terminal sequence handling, or remote PTY restore logic in app code.
- Keep generated build output out of version control.
