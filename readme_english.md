# AkiStash · Windows Desktop

<p align="center">
  <a href="README.md"><b>English</b></a> | <a href="README_zh.md"><b>简体中文</b></a>
</p>

AkiStash is a 2D / 3D drawing application. The name comes from Aki (the creator's nickname) and the autumn hoarding habit of hamsters: a little stash for storing colors, brushes, and inspirations.

This directory contains the desktop version bundled with [Tauri 2](https://v2.tauri.app/). For detailed feature explanations, refer to `功能说明.md`.

## Project Structure

```text
AkiStash/
├─ src/                      Web frontend (offline capable)
│  ├─ index.html             Main application entry
│  └─ three.min.js           Three.js r128 engine (local copy, no internet required)
├─ src-tauri/                Desktop wrapper
│  ├─ src/main.rs, lib.rs    Rust entries, registers dialogs and fs plugins
│  ├─ tauri.conf.json        App name, window dimensions, icons, bundle settings
│  ├─ capabilities/          Permissions: allows save dialogs & writing selected files
│  └─ icons/                 App icons (app-icon.png is the 1024×1024 source)
└─ .github/workflows/        GitHub Actions automated build workflow (optional)
```

## Prerequisites (One-time Setup)

1. **Rust**: If already installed, run `rustup update` in your terminal to update to the latest stable release.
2. **Microsoft C++ Build Tools**: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), and check "Desktop development with C++". If you already have Visual Studio with C++ workloads installed, you can skip this.
3. **WebView2**: Windows 10/11 comes with it pre-installed.
4. **Tauri CLI**:
   ```bash
   cargo install tauri-cli --version "^2" --locked
   ```
   (The initial compilation may take several minutes.)

## Development

Open a terminal in the `AkiStash` directory:

```bash
cargo tauri dev
```

The first run will download and compile dependencies. After that, an AkiStash window will pop up. Any changes to `src/index.html` can be previewed by restarting the application.

## Build Installer

```bash
cargo tauri build
```

Once completed, the installers are located at:
- `src-tauri/target/release/bundle/nsis/AkiStash_1.0.0_x64-setup.exe` (Recommended, installer with setup UI)
- `src-tauri/target/release/bundle/msi/AkiStash_1.0.0_x64_en-US.msi`

Send the `.exe` to others, and they can double-click to install it.

## Differences: Desktop vs. Web Version

- **Fully Offline**: The 3D engine is bundled locally; the system font (Microsoft YaHei) is used without fetching from the web.
- **File Saving**: "Save", "Export PNG", and "Export Brush" trigger the native Windows "Save As" dialog.
- **File Opening & Image Importing**: Native open file dialogs are supported; images can also be dragged directly onto the canvas or pasted via `Ctrl+V`.
- **Accidental Refresh Prevention**: F5 and `Ctrl+R` are disabled to prevent unsaved work from being lost.

## Modifying Version, Name, or Icon

- **Version**: Update `version` in both `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml`.
- **Icon**: Replace `src-tauri/icons/app-icon.png`, then run `cargo tauri icon src-tauri/icons/app-icon.png` to regenerate all sizes automatically.

## Automated Builds via GitHub Actions (Optional)

Push the repository to GitHub and tag a release version:

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions will automatically build on Windows runners and create a draft release in the repository with binaries attached. Free for public open-source repositories.

## About the "Windows protected your PC" Warning

Unsigned installers will trigger Windows SmartScreen showing "Unknown Publisher". Click **More info → Run anyway** to proceed. For production releases, you may:
- Apply for a free open-source code signing certificate via [SignPath Foundation](https://signpath.org/).
- Purchase a code signing certificate.

Once the installer is signed and gains sufficient downloads, this prompt will disappear.

## FAQ

- **Build error: cannot find `link.exe`**: C++ Build Tools are missing. Refer to Step 2 in "Prerequisites".
- **"Permission denied" when saving**: The save path is outside of user directories (Desktop / Documents / Downloads / Pictures). Add custom paths to `fs:scope` in `src-tauri/capabilities/default.json`.

## Acknowledgements

- Special thanks to **Zha Mao** for feature suggestions on digital painting, lasso bug hunting, and designing the logo o(*￣▽￣*)ブ
- Thanks to **rs** for filling in the engineering drawing capabilities.
- Thanks to Zha Mao's pet mouse for being featured in the logo!