# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## v0.7.0 (2023-05-23)
- Breaking Changes - added additional properties to `toast::ShowOptions`
- Features - added support for the Clipboard plugin

## v0.6.0 (2023-05-18)
- Breaking Changes - compatibility update for [capacitor 5.0](https://capacitorjs.com/docs/updating/5-0)
- Breaking Changes - `device.uuid` changed to `device.identifier`
- Features - added `ios_version` and `android_sdk_version` to `DeviceInfo` (these were new in Capacitor 5.0.0)

## v0.5.1 (2023-03-23)
- Bug Fixes - URLOpenListenerEvent now deserializes correctly on iOS

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
