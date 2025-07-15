# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## v0.13.0 (2025-07-25)

- Deprecated some fields for Capacitor 0.7

## v0.12.2 (2024-12-04)

- Features - Enabled Schedule::At for local notifications.

## v0.12.1 (2024-12-03)

- Features - Add App Launcher plugin support https://capacitorjs.com/docs/apis/app-launcher

## v0.12.0 (2024-07-10)

- Breaking Changes - Support capacitor 6
- Breaking Changes - Removed support for capacitor-rate-app and replaced it with in-app-review. The old plugin was deprecated and this is the recommended migration and has the exact same api. The name of the cargo feature has changed from `rate-plugin` to `review-plugin`

## v0.11.0 (2024-03-11)

- Features - added [admob plugin](https://github.com/capacitor-community/admob) behind the `admob_plugin` feature
- Features - added [browser plugin](https://capacitorjs.com/docs/apis/browser) - thanks [Soren Hansen](https://github.com/SorenHolstHansen)

## v0.10.0 (2023-12-05)

- Features - added [game plugin](https://github.com/openforge/capacitor-game-connect) behind the `game_plugin` feature

## v0.9.0 (2023-06-27)

- Bug Fixes - added defaults to many of the result types. This prevents some potential panics when fields are missing.
- Breaking Changes - moved `Error` and `PluginListenerHandle` to their own files. They are now accessible through the prelude.
- Breaking Changes - `SplashScreen` functions now require the `ios` or `android` feature.
- Features - added [rate plugin](https://github.com/Nodonisko/capacitor-rate-app) behind the `rate_plugin` feature

## v0.8.0 (2023-06-27)

- Features - added support for the SplashScreen plugin

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
