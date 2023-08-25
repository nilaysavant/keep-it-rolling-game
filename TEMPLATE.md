# Bevy game template

Adapting workflows/build/release setup etc from [bevy_game_template].

Featuring out of the box builds for Windows, Linux, macOS, and Web (Wasm). It also includes the setup for android support.

# Workflows

- workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
  - push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow
  - WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes. You might want to consider running the workflow less often or removing some builds from it. **For public repositories the builds are free!**

# How to use this template?

- Look for `ToDo` to use your own game name everywhere
- [Update the icons as described below](#updating-the-icons)
- Start coding :tada:
  - Start the native app: `cargo run`
  - Start the web build: `trunk serve`
    - requires [trunk]: `cargo install --locked trunk`
    - requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
    - this will serve your app on `8080` and automatically rebuild + reload it after code changes
  - Start the android app: `cargo apk run -p mobile` (update the library name if you changed it)
    - requires following the instructions in the [bevy example readme for android setup instructions][android-instructions]
  - Start the iOS app
    - Install Xcode through the app store
    - Launch Xcode and install the iOS simulator (check the box upon first start, or install it through `Preferences > Platforms` later)
    - Install the iOS and iOS simulator Rust targets with `rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim` (see the [bevy example readme for ios setup instructions][ios-instructions])
    - run `make run` inside the `/mobile` directory

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

### Updating the icons

1.  Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `build/macos` directory) - _Note: this requires a mac_
2.  Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
    - You can create an `.ico` file for windows by following these steps:
      1.  Open `macos/AppIcon.iconset/icon_256x256.png` in [Gimp](https://www.gimp.org/downloads/)
      2.  Select the `File > Export As` menu item.
      3.  Change the file extension to `.ico` (or click `Select File Type (By Extension)` and select `Microsoft Windows Icon`)
      4.  Save as `build/windows/icon.ico`
3.  Replace `build/android/res/mipmap-mdpi/icon.png` with `macos/AppIcon.iconset/icon_256x256.png`, but rename it to `icon.png`

### Deploy web build to GitHub pages

1.  Trigger the `deploy-github-page` workflow
2.  Activate [GitHub pages](https://pages.github.com/) for your repository
    1.  Source from the `gh-pages` branch (created by the just executed action)
3.  After a few minutes your game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

Note that this does a `cargo build` and thus does not work with local dependencies. Consider pushing your "custom Bevy fork" to GitHub and using it as a git dependency.

# Removing mobile platforms

If you don't want to target Android or iOS, you can just delete the `/mobile`, `/build/android`, and `/build/ios` directories.
Then delete the `[workspace]` section from `Cargo.toml`.

# Known issues

Audio in web-builds can have issues in some browsers. This seems to be a general performance issue and not due to the audio itself (see [bevy_kira_audio/#9][firefox-sound-issue]).

[bevy]: https://bevyengine.org/
[bevy-learn]: https://bevyengine.org/learn/
[bevy-discord]: https://discord.gg/bevy
[bevy_game_template]: https://github.com/NiklasEi/bevy_game_template
[firefox-sound-issue]: https://github.com/NiklasEi/bevy_kira_audio/issues/9
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[`wasm-server-runner`]: https://github.com/jakobhellermann/wasm-server-runner
[trunk]: https://trunkrs.dev/
[android-instructions]: https://github.com/bevyengine/bevy/blob/latest/examples/README.md#setup
[ios-instructions]: https://github.com/bevyengine/bevy/blob/latest/examples/README.md#setup-1
