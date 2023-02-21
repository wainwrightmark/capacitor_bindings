# capacitor_bindings

[<img alt="github" src="https://img.shields.io/badge/github-wainwrightmark/capacitor_bindings-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/wainwrightmark/capacitor_bindings)
[<img alt="crates.io" src="https://img.shields.io/crates/v/capacitor_bindings.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/capacitor_bindings)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/capacitor_bindings/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/capacitor_bindings)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/wainwrightmark/capacitor_bindings/build/main?style=for-the-badge" height="22">](https://github.com/wainwrightmark/capacitor_bindings/actions?query=branch%3Amain)

Capacitor bindings to help you build android and ios apps using rust.
These can be used from framework that builds with trunk e.g. yes, sycamore, seed, etc.

The workflow detailed below will let you have one application that builds for android, ios (probably - I haven't tested this yet), and the web.

This crate provides bindings for some of the [official capacitor plugins](https://capacitorjs.com/docs/plugins). I intend to support them all eventually. Please submit a pull request if there is a particular one you need.

Currently supported

- Action Sheet
- Camera
- Device
- Dialog
- Haptics
- Local Notifications (incomplete)
- Share
- Status Bar
- Toast

Some of the functions are only available on certain platforms. Use the features `web`, `android`, or `ios` to get access to them if you are building for that platform.

Run the example project to see all of the available features in action.

---

## How to use

```rust
use capacitor_bindings::toast::*;

async fn greet() {
    Toast::show("Hello World").await;
}
```

## Setup

This is how I set up my projects to be able to build multiple apps

### Prerequisites

You must do the [Capacitor environment setup](https://capacitorjs.com/docs/getting-started/environment-setup) installiong node, xcode and android studio as needed.

You must also have trunk installed

```
cargo install trunk
```

### Adding Capacitor to your app

Install Capacitor

Navigate to the directory containing your `index.html` file

Install Capacitor

```
npm i @capacitor/core
npm i -D @capacitor/cli
```

Initialize your app with the questionnaire.
Make sure the folder used is the same as your trunk output folder (e.g. `dist`)

```
npx cap init
```

Add android and ios as needed

```
npm i @capacitor/android @capacitor/ios
npx cap add android
npx cap add ios
```

Add the plugins that you want

```
npm install @capacitor/toast
```

Some plugins have additional installation steps (such as adding to the android manifest xml) that you will need to follow.
[the list of official plugins is here](https://capacitorjs.com/docs/plugins)

You should now be able to run your app on android (and possibly ios)

```
trunk build --release
npx cap sync
npx cap run android
```

You can aslo do `npx cap open android` to open the project in android studio.

### Enabling Capacitor on the web

You must likely want to be able run a web version of your app as well as the mobile version, if only for debugging.

The setup for this is slightly involved

First you need to generate a `capacitor.js` file. The easiest way to do this is to set `bundledWebRuntime` in your `capacitor.config.json` file to true. Then you can run `npx cap sync` and it will create `capacitor.js` in your `dist` folder. Move that into your root folder and set `bundledWebRuntime` back to false.

You then need to convert all the plugins you are using from npm modules into ordinary js files. The is a tool called pika that will do this for you.

```
npm install pika
```

```
npx @pika/web --dest plugins
```

Now you need to change your index.html to include all these files and also the [capacitor pwa elements](https://capacitorjs.com/docs/web/pwa-elements)

Add the following to your head section

```
<script src="capacitor.js" type="text/javascript"></script>
<script
    type="module"
    src="https://unpkg.com/@ionic/pwa-elements@latest/dist/ionicpwaelements/ionicpwaelements.esm.js"
></script>
<script
    nomodule
    src="https://unpkg.com/@ionic/pwa-elements@latest/dist/ionicpwaelements/ionicpwaelements.js"
></script>

<script src="./plugins/@capacitor/toast.js" type="module"></script>

<link data-trunk rel="copy-file" href="capacitor.js" />

<link data-trunk rel="copy-dir" href="plugins" />

```
Replace the `<script src="./plugins/@capacitor/toast.js" type="module"></script>` line with one line for each plugin you are using

If you now do `trunk serve` it should run in the browser and you will have access to the web versions of the plugins. This will also not break the android build but you will get error messages about those .js files not being available when you run on android.

To prevent this, and also to allow you to pass different features to each version, you should make a copy of your `index.html` and name in `android.html`. From this file you can remove all of the lines you just added to your head section. You can also control the features by changing the link in the body section.

```
<body>
    <link rel="rust" data-trunk href="Cargo.toml" data-bin="myapp" data-cargo-features="android" />
</body>
```


## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/wainwrightmark/capacitor_bindings/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`capacitor_bindings` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

- [Mark Wainwright](https://github.com/wainwrightmark)
