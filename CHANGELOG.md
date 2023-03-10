# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.#

## v0.5.0 (2023-03-14)
- Features - Added support for Preferences Plugin
- Features - Derive a typed builder for some types
- Breaking Changes - Added more support for Local Notifications

## v0.4.0 (2023-03-06)
- Features - Added support for Screen Reader plugin

## v0.3.0 (2023-03-05)
- Breaking Changes - PluginListenerHandle now works differently - must be stored to be dropped later or leaked.
- Features - Added support for App and Network plugins and more complete support of LocalNotifications

## v0.2.0 (2023-02-24)
- Breaking Changes - Made all functions return `Result<T, Error>`

## v0.1.0 (2023-02-21)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/capacitor_bindings
