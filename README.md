# capacitor_bindings

[<img alt="github" src="https://img.shields.io/badge/github-wainwrightmark/capacitor_bindings-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/wainwrightmark/capacitor_bindings)
[<img alt="crates.io" src="https://img.shields.io/crates/v/capacitor_bindings.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/capacitor_bindings)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/capacitor_bindings/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/capacitor_bindings)

Capacitor bindings to help you build android and ios apps using rust.

These can be used from framework that builds with trunk e.g. yes, sycamore, seed, etc.

**_This crate is still in the early stages. Use at your own risk. When capacitor make breaking changes, this crate will make breaking changes to stay up to date._**

I have successfully used this to build both iOS and Android apps.

In the Setup section below I explain how you can have a single version of your code and which can be built for both web and mobile platforms.

This crate provides bindings for some of the [official capacitor plugins](https://capacitorjs.com/docs/plugins). I intend to support them all eventually. Please submit an issue or pull request if there is a particular one you need.

Currently supported

- App
- Action Sheet
- Camera
- Device
- Dialog
- Haptics
- Local Notifications (incomplete)
- Network
- Preferences
- Share
- Status Bar
- Screen Reader
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

### Prerequisites

You must do the [Capacitor environment setup](https://capacitorjs.com/docs/getting-started/environment-setup) installiong node, xcode and android studio as needed.

You must also have trunk installed

```
cargo install trunk
```

You must have a rust web app that builds with trunk. You can try it out using the example application in this repository.

### Adding Capacitor to your app

Install Capacitor

Navigate to the directory containing your `index.html` file

Install Capacitor

```
npm i @capacitor/core
npm i -D @capacitor/cli
```

Initialize your app with the questionnaire.
You will have to answer some questions and decide the name of your app.
Make sure the "web asset directory" is the same as your trunk output folder (usually `dist`)

```
npx cap init
```

You should probably add the following to your .gitignore as some npm files do not need to be committed

```
## Node
node_modules
package-lock.json
package.json
```

Add android and ios as needed

```
npm i @capacitor/android
npx cap add android

npm i @capacitor/ios
npx cap add ios
```

Add the plugins that you want e.g.

```
npm install @capacitor/toast
```

Add this crate to your project

```
cargo add capacitor_bindings --features web,android,ios
```

Add calls to the plugins where you need them. If you have a yew project, you could add the following to one of your `function_component`

```rust
    use_effect(||yew::platform::spawn_local(capacitor_bindings::toast::Toast::show("Hello World")));
```

Some plugins have additional installation steps (such as adding to the android manifest xml) that you will need to follow.
[the list of official plugins is here](https://capacitorjs.com/docs/plugins)

You should now be able to run your app on android or ios

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
Alternatively, just copy the file from the example application in this repository.

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

To prevent this, and also to allow you to pass different features to each version, you should make a copy of your `index.html` and name in `android.html`. From this file you can remove all of the lines you just added to your head section.

### Features

Some of the plugin functions only work on certain platforms. For example setting the status bar style doesn't work on the web.

To enable these features for the appropriate builds, add the following to your cargo.toml

```
[features]
web =["capacitor_bindings/web"]
android = ["capacitor_bindings/android"]
ios = ["capacitor_bindings/ios"]
```

To only use a feature when it is availble you need to have a section of your rust code that is only build when that feature is enabled.

```rust
#[cfg(feature="android")]
{
    StatusBar::set_style(Style::Light).await;
    StatusBar::set_background_color("#FFFFFF").await;
}
```

You can control which features trunk will build by using the `data-cargo-features` attribute in the link tag in your `index.html` or `android.html`

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
